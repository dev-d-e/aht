use crate::error::*;
use crate::markup::*;
use crate::utils::*;
use ffmpeg_next::codec::context::Context;
use ffmpeg_next::decoder::{Audio, Video};
use ffmpeg_next::ffi::*;
use ffmpeg_next::format::context::Input;
use ffmpeg_next::format::{Pixel, Sample, input};
use ffmpeg_next::frame::{Audio as AudioFrame, Video as VideoFrame};
use ffmpeg_next::media::Type as MediaType;
use ffmpeg_next::rescale::TIME_BASE;
use ffmpeg_next::software::resampling::Context as SwrContext;
use ffmpeg_next::software::scaling::Context as SwsContext;
use ffmpeg_next::{Packet, Rational, Rescale};
use skia_safe::{ColorType, Data, ISize, Image, ImageInfo, SamplingOptions, Size, images};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::Barrier;
use tokio::sync::broadcast::{
    Receiver as BroadcastReceiver, Sender as BroadcastSender, channel as broadcast_channel,
};
use tokio::sync::mpsc::{
    Receiver as MpscReceiver, Sender as MpscSender, channel as mpsc_channel, error::TrySendError,
};
use tokio::time::{Duration, Interval, MissedTickBehavior, interval};

const CTRL_SIZE: usize = 8;

const VIDEO_PACKET_SIZE: usize = 1000;
const IMAGE_DATA_SIZE: usize = 100;

const AUDIO_PACKET_SIZE: usize = 8;
const SOUND_DATA_SIZE: usize = 8;

fn interval_millis() -> Interval {
    let mut interval = interval(Duration::from_millis(100));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    interval
}

#[cfg(target_os = "linux")]
mod linux;

struct ImageData {
    width: f32,
    height: f32,
    bytes: VideoFrame,
}

impl ImageData {
    fn new(bytes: VideoFrame) -> Self {
        Self {
            width: bytes.width() as f32,
            height: bytes.height() as f32,
            bytes,
        }
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.data(0)
    }

    fn to_size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    fn to_isize(&self) -> ISize {
        ISize::new(self.width as i32, self.height as i32)
    }

    fn equals(&self, r: &RectSide) -> bool {
        self.width == r.width() && self.height == r.height()
    }

    fn to_data(&self) -> Data {
        unsafe { Data::new_bytes(self.bytes()) }
    }

    fn row_bytes(&self, size: usize) -> usize {
        self.width as usize * size
    }

    fn to_image(self, image_info: ImageInfo, r: &RectSide) -> Option<Image> {
        let info = image_info.with_dimensions(self.to_isize());
        let info = info.with_color_type(ColorType::RGBA8888);
        let i = images::raster_from_data(&info, self.to_data(), self.row_bytes(4))?;
        if self.equals(r) {
            Some(i)
        } else {
            let info = info.with_dimensions(r);
            i.make_scaled(&info, SamplingOptions::default())
        }
    }
}

impl From<VideoFrame> for ImageData {
    fn from(o: VideoFrame) -> Self {
        Self::new(o)
    }
}

#[derive(Debug)]
pub(crate) struct ImageDataOutput {
    buffer: Option<Image>,
    data_receiver: Option<MpscReceiver<ImageData>>,
    pause_sender: BroadcastSender<bool>,
    seek_sender: MpscSender<(u32, u32)>,
    close_sender: MpscSender<()>,
}

impl ImageDataOutput {
    fn new(
        data_receiver: Option<MpscReceiver<ImageData>>,
        pause_sender: BroadcastSender<bool>,
        seek_sender: MpscSender<(u32, u32)>,
        close_sender: MpscSender<()>,
    ) -> Self {
        Self {
            buffer: None,
            data_receiver,
            pause_sender,
            seek_sender,
            close_sender,
        }
    }

    pub(crate) fn data(&mut self, image_info: ImageInfo, r: &RectSide) -> &Option<Image> {
        if let Some(receiver) = &mut self.data_receiver {
            if let Ok(data) = receiver.try_recv() {
                if let Some(i) = data.to_image(image_info, r) {
                    let o = self.buffer.replace(i);
                    drop(o);
                }
            }
        }
        return &self.buffer;
    }

    pub(crate) fn pause(&mut self, o: bool) -> bool {
        self.pause_sender.send(o).is_ok()
    }

    pub(crate) fn seek(&mut self, o: (u32, u32)) -> bool {
        self.seek_sender.try_send(o).is_ok()
    }
}

impl Drop for ImageDataOutput {
    fn drop(&mut self) {
        let _ = self.close_sender.try_send(());
        if let Some(receiver) = &mut self.data_receiver {
            receiver.close();
        }
        info!("ImageDataOutput close");
    }
}

struct VideoConsumer {
    decoder: Video,
    sws_context: Option<SwsContext>,
    stream: StreamData,
    rate: Rational,
    v_packet: VecDeque<Packet>,
    v_frame: VecDeque<VideoFrame>,
    packet_receiver: MpscReceiver<Packet>,
    data_sender: MpscSender<ImageData>,
    time_receiver: MpscReceiver<i64>,
    pause_receiver: BroadcastReceiver<bool>,
    seek_receiver: MpscReceiver<Arc<Barrier>>,
}

impl VideoConsumer {
    fn new(
        decoder: Video,
        stream: StreamData,
        rate: Rational,
        packet_receiver: MpscReceiver<Packet>,
        data_sender: MpscSender<ImageData>,
        time_receiver: MpscReceiver<i64>,
        pause_receiver: BroadcastReceiver<bool>,
        seek_receiver: MpscReceiver<Arc<Barrier>>,
    ) -> Self {
        let sws_context = decoder
            .converter(Pixel::RGBA)
            .inspect_err(|e| error!("sws_context: {e:?}"))
            .ok();

        Self {
            decoder,
            sws_context,
            stream,
            rate,
            v_packet: VecDeque::with_capacity(VIDEO_PACKET_SIZE),
            v_frame: VecDeque::with_capacity(IMAGE_DATA_SIZE),
            packet_receiver,
            data_sender,
            time_receiver,
            pause_receiver,
            seek_receiver,
        }
    }

    fn video_frame(&mut self, frame: VideoFrame) {
        if frame.format() == Pixel::RGBA {
            self.v_frame.push_back(frame);
        } else if let Some(s) = &mut self.sws_context {
            let mut out = VideoFrame::empty();
            if let Err(e) = s.run(&frame, &mut out) {
                drop(out);
                warn!("sws_context run: {e:?}");
            } else {
                out.set_pts(frame.pts().or(frame.timestamp()));
                self.v_frame.push_back(out);
            }
        }
    }

    fn clear(&mut self) {
        self.decoder.flush();
        while let Ok(o) = self.packet_receiver.try_recv() {
            drop(o);
        }
        self.v_packet.clear();
        self.v_frame.clear();
    }

    async fn consume(mut self) {
        let n = self.rate.denominator() as u64 * 1000000 / self.rate.numerator() as u64;
        let mut interval = interval(Duration::from_micros(n));
        let mut time = None;
        let mut wait: u64 = 0;
        let mut pause = false;
        let mut send = false;
        loop {
            tokio::select! {
                biased;
                r = self.time_receiver.recv() => {
                    if let Some(o) = r {
                        time.replace(o);
                        info!("VideoConsumer time");
                    }
                }
                r = self.pause_receiver.recv() => {
                    if let Ok(o) = r {
                        pause = o;
                        info!("VideoConsumer pause");
                    }
                }
                r = self.seek_receiver.recv() => {
                    if let Some(o) = r {
                        self.clear();
                        o.wait().await;
                        info!("VideoConsumer seek");
                    }
                }
                _ = interval.tick() => {
                    if !pause {
                        if wait > 0 {
                            info!("VideoConsumer wait");
                            wait = wait.saturating_sub(n);
                        } else {
                            send = true;
                        }
                    }
                }
                r = self.packet_receiver.recv(), if self.v_packet.len() < VIDEO_PACKET_SIZE => {
                    if let Some(o) = r {
                        self.v_packet.push_back(o);
                    }
                }
            };
            if send {
                send = false;
                trace!(
                    "VideoConsumer vec: {} {}",
                    self.v_packet.len(),
                    self.v_frame.len()
                );
                while let Some(frame) = self.v_frame.pop_front() {
                    if let Some(a) = time.take() {
                        if let Some(pts) = frame.pts() {
                            let b = self.stream.pts_to_micros(pts);
                            info!("VideoConsumer sync {} {}", a, b);
                            if b + 10000 < a {
                                drop(frame);
                                time.replace(a);
                                continue;
                            } else if a + 10000 < b {
                                wait = (b - a) as u64;
                                self.v_frame.push_front(frame);
                                break;
                            }
                        }
                    }
                    if let Err(e) = self.data_sender.try_send(frame.into()) {
                        if matches!(e, TrySendError::Closed(_)) {
                            debug!("VideoConsumer Close");
                            return;
                        }
                    }
                    break;
                }
            }
            if self.v_frame.len() < IMAGE_DATA_SIZE {
                if let Some(packet) = self.v_packet.pop_front() {
                    if let Err(e) = self.decoder.send_packet(&packet) {
                        trace!("VideoConsumer send_packet: {e:?}");
                    } else {
                        loop {
                            let mut frame = VideoFrame::empty();
                            if let Err(e) = self.decoder.receive_frame(&mut frame) {
                                drop(frame);
                                trace!("VideoConsumer receive_frame: {e:?}");
                                break;
                            } else {
                                self.video_frame(frame);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct SoundData {
    bytes: AudioFrame,
    i: usize,
}

impl SoundData {
    fn new(bytes: AudioFrame) -> Self {
        Self { bytes, i: 0 }
    }

    fn bytes(&self) -> &[u8] {
        &self.bytes.data(0)[self.i..]
    }

    fn has_data(&mut self, n: usize) -> bool {
        let n = self.i + n;
        let r = n < self.bytes.data(0).len();
        if r {
            self.i = n;
        }
        r
    }
}

impl From<AudioFrame> for SoundData {
    fn from(o: AudioFrame) -> Self {
        Self::new(o)
    }
}

#[derive(Debug)]
struct SoundDataOutput {
    stream: StreamData,
    v_data: VecDeque<SoundData>,
    data_receiver: MpscReceiver<SoundData>,
    pause_receiver: BroadcastReceiver<bool>,
    seek_receiver: MpscReceiver<Arc<Barrier>>,
    close_receiver: MpscReceiver<Arc<Barrier>>,
}

impl Drop for SoundDataOutput {
    fn drop(&mut self) {
        self.data_receiver.close();
        info!("SoundDataOutput close");
    }
}

struct AudioConsumer {
    decoder: Audio,
    swr_context: Option<SwrContext>,
    v_packet: VecDeque<Packet>,
    v_frame: VecDeque<AudioFrame>,
    packet_receiver: MpscReceiver<Packet>,
    data_sender: MpscSender<SoundData>,
    seek_receiver: MpscReceiver<Arc<Barrier>>,
    seek_sender: MpscSender<Arc<Barrier>>,
}

impl AudioConsumer {
    fn new(
        decoder: Audio,
        stream: StreamData,
        packet_receiver: MpscReceiver<Packet>,
        pause_receiver: BroadcastReceiver<bool>,
        seek_receiver: MpscReceiver<Arc<Barrier>>,
        close_receiver: MpscReceiver<Arc<Barrier>>,
    ) -> Self {
        let (data_sender, data_receiver) = mpsc_channel(1);
        let (seek_sender1, seek_receiver1) = mpsc_channel(CTRL_SIZE);

        let output = SoundDataOutput {
            stream,
            v_data: VecDeque::with_capacity(SOUND_DATA_SIZE),
            data_receiver,
            pause_receiver,
            seek_receiver: seek_receiver1,
            close_receiver,
        };

        let p = SoundProperty::new(&decoder);

        let swr_context = decoder
            .resampler(p.format, decoder.channel_layout(), p.rate)
            .inspect_err(|e| error!("swr_context: {e:?}"))
            .ok();

        let handle = Handle::current();
        std::thread::spawn(move || {
            #[cfg(target_os = "linux")]
            handle.block_on(async {
                if let Some(mut sound) = linux::Sound::new(p) {
                    sound.playback(output).await;
                }
            });

            #[cfg(not(any(target_os = "linux", target_os = "ios", target_os = "android")))]
            handle.block_on(async {
                let mut receiver = receiver;
                while let Ok(data) = receiver.data.try_recv() {
                    drop(data);
                }
            });
        });

        Self {
            decoder,
            swr_context,
            v_packet: VecDeque::with_capacity(AUDIO_PACKET_SIZE),
            v_frame: VecDeque::with_capacity(SOUND_DATA_SIZE),
            packet_receiver,
            data_sender,
            seek_receiver,
            seek_sender: seek_sender1,
        }
    }

    fn audio_frame(&mut self, frame: AudioFrame) {
        if frame.is_planar() {
            if let Some(s) = &mut self.swr_context {
                let mut out = AudioFrame::empty();
                match s.run(&frame, &mut out) {
                    Ok(_) => {
                        out.set_pts(frame.pts().or(frame.timestamp()));
                        self.v_frame.push_back(out);
                    }
                    Err(e) => {
                        drop(out);
                        warn!("swr_context run: {e:?}");
                    }
                }
            }
        } else {
            self.v_frame.push_back(frame);
        }
    }

    fn clear(&mut self) {
        self.decoder.flush();
        while let Ok(o) = self.packet_receiver.try_recv() {
            drop(o);
        }
        self.v_packet.clear();
        self.v_frame.clear();
    }

    async fn consume(mut self) {
        let mut interval = interval_millis();
        loop {
            tokio::select! {
                biased;
                r = self.seek_receiver.recv() => {
                    if let Some(o) = r {
                        self.clear();
                        let o2 = Arc::new(Barrier::new(2));
                        if let Err(e) = self.seek_sender.send(o2.clone()).await{
                            drop(e);
                        }
                        o2.wait().await;
                        o.wait().await;
                        info!("AudioConsumer seek");
                    }
                }
                _ = interval.tick() => {
                    trace!("AudioConsumer vec: {} {}", self.v_packet.len(), self.v_frame.len());
                }
                p = self.packet_receiver.recv(), if self.v_packet.len() < AUDIO_PACKET_SIZE => {
                    if let Some(packet) = p {
                        self.v_packet.push_back(packet);
                    }
                }
            };
            if self.v_frame.len() > 0 {
                loop {
                    match self.data_sender.try_reserve() {
                        Ok(o) => {
                            if let Some(frame) = self.v_frame.pop_front() {
                                o.send(frame.into());
                            } else {
                                break;
                            }
                        }
                        Err(e) => {
                            if matches!(e, TrySendError::Closed(_)) {
                                debug!("AudioConsumer Close");
                                return;
                            }
                        }
                    }
                }
            }
            if self.v_frame.len() < SOUND_DATA_SIZE {
                if let Some(packet) = self.v_packet.pop_front() {
                    if let Err(e) = self.decoder.send_packet(&packet) {
                        trace!("AudioConsumer send_packet: {e:?}");
                    } else {
                        loop {
                            let mut frame = AudioFrame::empty();
                            if let Err(e) = self.decoder.receive_frame(&mut frame) {
                                drop(frame);
                                trace!("AudioConsumer receive_frame: {e:?}");
                                break;
                            } else {
                                self.audio_frame(frame);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct StreamData {
    index: usize,
    time_base: Rational,
    start: i64,
    duration: i64,
}

impl StreamData {
    fn index(&self) -> i32 {
        self.index as i32
    }

    fn pts_to_micros(&self, pts: i64) -> i64 {
        (pts - self.start).rescale(self.time_base, Rational::new(1, 1000000))
    }

    fn seek_pts(&self, a: i64, b: i64) -> i64 {
        self.duration * a / b + self.start
    }
}

impl From<(usize, Rational, i64, i64)> for StreamData {
    fn from(o: (usize, Rational, i64, i64)) -> Self {
        Self {
            index: o.0,
            time_base: o.1,
            start: o.2,
            duration: o.3,
        }
    }
}

///MediaWrapper.
struct MediaWrapper {
    input: Input,
    map: HashMap<usize, MpscSender<Packet>>,
    video_stream: Option<StreamData>,
    audio_stream: Option<StreamData>,
}

impl MediaWrapper {
    fn new(p: &str) -> Result<Self> {
        input(p)
            .map_err(|e| to_err(ErrorKind::Media, e))
            .map(|input| Self {
                input,
                map: HashMap::new(),
                video_stream: None,
                audio_stream: None,
            })
    }

    async fn decode(&mut self, sender: MpscSender<ImageDataOutput>, is_output: bool) {
        let duration = self.input.duration();
        let streams = self.input.streams();

        let mut time_sender_holder = None;
        let (pause_sender, pause_receiver) = broadcast_channel(CTRL_SIZE);
        let (seek_sender, mut seek_receiver) = mpsc_channel(CTRL_SIZE);
        let mut seek_sender_holder = Vec::with_capacity(3);
        let (close_sender, mut close_receiver) = mpsc_channel(CTRL_SIZE);
        let mut close_sender_holder = None;

        if is_output && let Some(s) = streams.best(MediaType::Video) {
            match Context::from_parameters(s.parameters())
                .map(|context| context.decoder().video())
                .flatten()
            {
                Ok(decoder) => {
                    let index = s.index();
                    let time_base = s.time_base();
                    let o = (
                        index,
                        time_base,
                        s.start_time(),
                        duration.rescale(TIME_BASE, time_base),
                    );
                    info!("video stream: {:?}", o);
                    self.video_stream.replace(o.into());

                    let (packet_sender, packet_receiver) = mpsc_channel(1);
                    self.map.insert(index, packet_sender);

                    let rate = s.avg_frame_rate();
                    let (data_sender, data_receiver) = mpsc_channel(1);
                    let (time_sender, time_receiver) = mpsc_channel(CTRL_SIZE);
                    time_sender_holder.replace(time_sender);
                    let pause_receiver = pause_receiver.resubscribe();
                    let (seek_sender1, seek_receiver1) = mpsc_channel(CTRL_SIZE);
                    seek_sender_holder.push(seek_sender1);
                    let handle = Handle::current();
                    std::thread::spawn(move || {
                        handle.block_on(async {
                            VideoConsumer::new(
                                decoder,
                                o.into(),
                                rate,
                                packet_receiver,
                                data_sender,
                                time_receiver,
                                pause_receiver,
                                seek_receiver1,
                            )
                            .consume()
                            .await;
                        });
                    });

                    let o = ImageDataOutput::new(
                        Some(data_receiver),
                        pause_sender,
                        seek_sender,
                        close_sender,
                    );
                    let _ = sender.send(o).await;
                }
                Err(e) => {
                    error!("video stream: {e:?}");
                }
            }
        } else {
            let o = ImageDataOutput::new(None, pause_sender, seek_sender, close_sender);
            let _ = sender.send(o).await;
        }

        if let Some(s) = streams.best(MediaType::Audio) {
            match Context::from_parameters(s.parameters())
                .map(|context| context.decoder().audio())
                .flatten()
            {
                Ok(decoder) => {
                    let index = s.index();
                    let time_base = s.time_base();
                    let o = (
                        index,
                        time_base,
                        s.start_time(),
                        duration.rescale(TIME_BASE, time_base),
                    );
                    info!("audio stream: {:?}", o);
                    self.audio_stream.replace(o.into());

                    let (packet_sender, packet_receiver) = mpsc_channel(1);
                    self.map.insert(index, packet_sender);

                    let (seek_sender1, seek_receiver1) = mpsc_channel(CTRL_SIZE);
                    seek_sender_holder.push(seek_sender1);
                    let (close_sender, close_receiver) = mpsc_channel(1);
                    close_sender_holder.replace(close_sender);
                    let handle = Handle::current();
                    std::thread::spawn(move || {
                        handle.block_on(async {
                            AudioConsumer::new(
                                decoder,
                                o.into(),
                                packet_receiver,
                                pause_receiver,
                                seek_receiver1,
                                close_receiver,
                            )
                            .consume()
                            .await;
                        });
                    });
                }
                Err(e) => {
                    error!("audio stream: {e:?}");
                }
            }
        } else {
            drop(pause_receiver);
        }

        info!("start packet");
        let mut packet_holder: Option<Packet> = None;
        let mut seek = None;
        let mut time = false;
        let mut close = false;
        loop {
            if let Ok(n) = seek_receiver.try_recv() {
                seek.replace(n);
            }
            if let Some((a, b)) = seek.take() {
                if let Some(o) = self.video_stream.as_ref().or(self.audio_stream.as_ref()) {
                    let i = o.index();
                    let t = o.seek_pts(a as i64, b as i64);
                    unsafe {
                        let p = self.input.as_mut_ptr();
                        let f = AVSEEK_FLAG_BACKWARD;
                        let r = avformat_seek_file(p, i, 0, t, i64::MAX, f);
                        if r < 0 {
                            warn!("MediaWrapper seek: {:?}", ffmpeg_next::Error::from(r));
                            continue;
                        }
                    }
                    info!("MediaWrapper seek");
                    let barrier = Arc::new(Barrier::new(seek_sender_holder.len() + 1));
                    for sender in &seek_sender_holder {
                        if let Err(e) = sender.send(barrier.clone()).await {
                            drop(e);
                        }
                        time = true;
                    }
                    barrier.wait().await;
                }
            }
            if let Ok(_) = close_receiver.try_recv() {
                close = true;
            }
            if close {
                if let Some(sender) = &close_sender_holder {
                    let barrier = Arc::new(Barrier::new(2));
                    if let Err(e) = sender.send(barrier.clone()).await {
                        drop(e);
                    }
                    barrier.wait().await;
                }
                info!("MediaWrapper close");
                return;
            }
            if let Some(packet) = packet_holder.take() {
                if let Some(packet_sender) = self.map.get_mut(&packet.stream()) {
                    if let Ok(o) = packet_sender.try_reserve() {
                        o.send(packet);
                        continue;
                    }
                }
                packet_holder.replace(packet);
                continue;
            }
            let mut packet = Packet::empty();
            if let Err(e) = packet.read(&mut self.input) {
                if matches!(e, ffmpeg_next::Error::Eof) {
                    debug!("MediaWrapper packet read eof");
                    tokio::select! {
                        r = seek_receiver.recv() => {
                            if let Some(n) =r {
                                seek.replace(n);
                            }
                        }
                        r = close_receiver.recv() => {
                            if let Some(_) = r {
                                close = true;
                            }
                        }
                    };
                    continue;
                } else {
                    debug!("MediaWrapper packet read: {e:?}");
                }
            } else {
                if time {
                    if let Some(s) = &self.audio_stream {
                        if s.index == packet.stream() {
                            time = false;
                            if let Some(time_sender) = &time_sender_holder {
                                if let Some(n) = packet.pts() {
                                    let _ = time_sender.try_send(s.pts_to_micros(n));
                                }
                            }
                        }
                    }
                }
                if let Some(packet_sender) = self.map.get_mut(&packet.stream()) {
                    if let Ok(o) = packet_sender.try_reserve() {
                        o.send(packet);
                    } else {
                        packet_holder.replace(packet);
                    }
                }
            }
        }
    }
}

pub(crate) fn build(s: String, is_output: bool) -> MpscReceiver<ImageDataOutput> {
    let (sender, receiver) = mpsc_channel(1);
    std::thread::spawn(move || {
        if let Err(e) = Runtime::new().map(|rt| {
            rt.block_on(async {
                match MediaWrapper::new(&s) {
                    Ok(mut w) => {
                        w.decode(sender, is_output).await;
                    }
                    Err(e) => {
                        error!("media: {e}");
                    }
                }
            });
        }) {
            error!("Runtime: {e:?}");
        }
    });
    receiver
}

#[derive(Debug)]
struct SoundProperty {
    channels: u32,
    rate: u32,
    format: Sample,
    period_size: i64,
    buffer_size: i64,
}

impl SoundProperty {
    fn new(decoder: &Audio) -> Self {
        Self {
            channels: decoder.channels() as u32,
            rate: decoder.rate(),
            format: decoder.format().packed(),
            period_size: 0,
            buffer_size: 0,
        }
    }
}
