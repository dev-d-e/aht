use crate::error::*;
use crate::markup::*;
use crate::utils::*;
use ffmpeg_next::codec::context::Context;
use ffmpeg_next::decoder::{Audio, Video};
use ffmpeg_next::ffi::*;
use ffmpeg_next::format::context::Input;
use ffmpeg_next::format::{input, Pixel, Sample};
use ffmpeg_next::frame::{Audio as AudioFrame, Video as VideoFrame};
use ffmpeg_next::media::Type as MediaType;
use ffmpeg_next::rescale::TIME_BASE;
use ffmpeg_next::software::resampling::Context as SwrContext;
use ffmpeg_next::software::scaling::Context as SwsContext;
use ffmpeg_next::{Packet, Rescale, Stream};
use getset::{Getters, MutGetters};
use skia_safe::{images, ColorType, Data, ISize, Image, ImageInfo, SamplingOptions, Size};
use std::collections::HashMap;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::broadcast::{
    channel as broadcast_channel, Receiver as BroadcastReceiver, Sender as BroadcastSender,
};
use tokio::sync::mpsc::{channel as mpsc_channel, Receiver as MpscReceiver, Sender as MpscSender};

const CTRL_SIZE: usize = 8;

const VIDEO_PACKET_SIZE: usize = 256;
const IMAGE_DATA_SIZE: usize = 8;
const IMAGE_DATA_FPS: f32 = 24.0;

const AUDIO_PACKET_SIZE: usize = 32;
const SOUND_DATA_SIZE: usize = 1;

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

#[derive(Clone, Debug)]
enum ImageCtrl {
    Seek(f32),
    Pause(bool),
    Close,
}

#[derive(Debug)]
struct ImageDataSender {
    data: MpscSender<ImageData>,
    ctrl: BroadcastReceiver<ImageCtrl>,
}

impl ImageDataSender {
    fn new(data: MpscSender<ImageData>, ctrl: BroadcastReceiver<ImageCtrl>) -> Self {
        Self { data, ctrl }
    }
}

#[derive(Debug, Getters, MutGetters)]
pub(crate) struct ImageDataReceiver {
    data: MpscReceiver<ImageData>,
    buffer: Option<Image>,
    ctrl: BroadcastSender<ImageCtrl>,
    fps_ctrl: FpsCtrl,
    pause: bool,
}

impl ImageDataReceiver {
    fn new(data: MpscReceiver<ImageData>, ctrl: BroadcastSender<ImageCtrl>) -> Self {
        Self {
            data,
            buffer: None,
            ctrl,
            fps_ctrl: FpsCtrl::new(IMAGE_DATA_FPS),
            pause: false,
        }
    }

    pub(crate) fn data(&mut self, image_info: ImageInfo, r: &RectSide) -> &Option<Image> {
        if !self.pause {
            if self.fps_ctrl.is_next() {
                if let Ok(data) = self.data.try_recv() {
                    if let Some(i) = data.to_image(image_info, r) {
                        let o = self.buffer.replace(i);
                        drop(o);
                    }
                }
            }
        }
        return &self.buffer;
    }

    pub(crate) fn none(&mut self) {
        if let Ok(data) = self.data.try_recv() {
            drop(data)
        }
    }

    fn ctrl(&mut self, c: ImageCtrl) -> bool {
        self.ctrl.send(c).is_ok()
    }

    pub(crate) fn ctrl_seek(&mut self, o: f32) -> bool {
        self.ctrl(ImageCtrl::Seek(o))
    }

    pub(crate) fn ctrl_pause(&mut self, o: bool) -> bool {
        self.pause = o;
        self.ctrl(ImageCtrl::Pause(o))
    }

    pub(crate) fn ctrl_close(&mut self) {
        self.ctrl(ImageCtrl::Close);
    }
}

fn image_build_channel() -> (ImageDataSender, ImageDataReceiver) {
    let (sender, receiver) = mpsc_channel(IMAGE_DATA_SIZE);
    let (sender2, receiver2) = broadcast_channel(CTRL_SIZE);
    (
        ImageDataSender::new(sender, receiver2),
        ImageDataReceiver::new(receiver, sender2),
    )
}

struct VideoConsumer {
    decoder: Video,
    sws_context: Option<SwsContext>,
    packet_receiver: MpscReceiver<Packet>,
    sender: ImageDataSender,
}

impl VideoConsumer {
    fn new(decoder: Video, packet_receiver: MpscReceiver<Packet>, sender: ImageDataSender) -> Self {
        let sws_context = decoder
            .converter(Pixel::RGBA)
            .inspect_err(|e| error!("sws_context: {e:?}"))
            .ok();

        Self {
            decoder,
            sws_context,
            packet_receiver,
            sender,
        }
    }

    async fn send_frame(&mut self, frame: VideoFrame) {
        if let Err(e) = self.sender.data.send(frame.into()).await {
            debug!("{e:?}");
        }
    }

    async fn video_frame(&mut self, mut video_frame: VideoFrame) {
        let timestamp = video_frame.timestamp();
        video_frame.set_pts(timestamp);

        if video_frame.format() == Pixel::RGBA {
            self.send_frame(video_frame).await;
        } else {
            if let Some(s) = &mut self.sws_context {
                let mut out = VideoFrame::empty();
                match s.run(&video_frame, &mut out) {
                    Ok(_) => {
                        self.send_frame(out).await;
                    }
                    Err(e) => {
                        drop(out);
                        debug!("{e:?}");
                    }
                }
                drop(video_frame);
            }
        }
    }

    async fn packet(&mut self, packet: Packet) {
        if let Err(e) = self.decoder.send_packet(&packet) {
            debug!("send video packet: {e:?}");
            return;
        }
        loop {
            let mut decoded = VideoFrame::empty();
            match self.decoder.receive_frame(&mut decoded) {
                Ok(_) => {
                    self.video_frame(decoded).await;
                }
                Err(e) => {
                    drop(decoded);
                    debug!("{e:?}");
                    break;
                }
            }
        }
    }

    fn clear(&mut self) {
        self.decoder.flush();
        let n = self.packet_receiver.len();
        if n > 0 {
            for _ in 0..n {
                if let Ok(p) = self.packet_receiver.try_recv() {
                    drop(p);
                }
            }
        }
    }

    fn end(self) {}

    async fn consume(mut self) {
        loop {
            tokio::select! {
                p = self.packet_receiver.recv() => {
                    if let Some(packet) = p {
                        self.packet(packet).await;
                    } else {
                        break;
                    }
                }
                r = self.sender.ctrl.recv() => {
                    match r {
                        Ok(o) => {
                            match o {
                                ImageCtrl::Seek(_) => {
                                    self.clear();
                                }
                                ImageCtrl::Pause(_) => {}
                                ImageCtrl::Close => {
                                    break;
                                }
                            }
                        },
                        Err(e) => {
                            debug!("{e}");
                        },
                    }
                }
            };
        }
        self.end();
    }
}

#[derive(Debug)]
struct SoundData {
    bytes: AudioFrame,
}

impl SoundData {
    fn new(bytes: AudioFrame) -> Self {
        Self { bytes }
    }

    fn bytes(&self) -> &[u8] {
        self.bytes.data(0)
    }
}

impl From<AudioFrame> for SoundData {
    fn from(o: AudioFrame) -> Self {
        Self::new(o)
    }
}

#[derive(Debug)]
struct SoundDataSender {
    data: MpscSender<SoundData>,
    ctrl: BroadcastReceiver<ImageCtrl>,
}

#[derive(Debug)]
struct SoundDataReceiver {
    data: MpscReceiver<SoundData>,
    ctrl: BroadcastReceiver<ImageCtrl>,
}

fn sound_build_channel(ctrl: BroadcastReceiver<ImageCtrl>) -> (SoundDataSender, SoundDataReceiver) {
    let (sender, receiver) = mpsc_channel(SOUND_DATA_SIZE);
    (
        SoundDataSender {
            data: sender,
            ctrl: ctrl.resubscribe(),
        },
        SoundDataReceiver {
            data: receiver,
            ctrl,
        },
    )
}

struct AudioConsumer {
    decoder: Audio,
    swr_context: Option<SwrContext>,
    packet_receiver: MpscReceiver<Packet>,
    sender: SoundDataSender,
}

impl AudioConsumer {
    fn new(
        decoder: Audio,
        packet_receiver: MpscReceiver<Packet>,
        ctrl: BroadcastReceiver<ImageCtrl>,
    ) -> Self {
        let (sender, receiver) = sound_build_channel(ctrl);

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
                    sound.playback(receiver).await;
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
            packet_receiver,
            sender,
        }
    }

    async fn send_frame(&mut self, frame: AudioFrame) {
        if let Err(e) = self.sender.data.send(frame.into()).await {
            debug!("{e:?}");
        }
    }

    async fn audio_frame(&mut self, audio_frame: AudioFrame) {
        if audio_frame.is_planar() {
            if let Some(s) = &mut self.swr_context {
                let mut out = AudioFrame::empty();
                match s.run(&audio_frame, &mut out) {
                    Ok(_) => {
                        self.send_frame(out).await;
                    }
                    Err(e) => {
                        drop(out);
                        debug!("{e:?}");
                    }
                }
                drop(audio_frame);
            }
        } else {
            self.send_frame(audio_frame).await;
        }
    }

    async fn packet(&mut self, packet: Packet) {
        if let Err(e) = self.decoder.send_packet(&packet) {
            debug!("send audio packet: {e:?}");
            return;
        }
        loop {
            let mut decoded = AudioFrame::empty();
            match self.decoder.receive_frame(&mut decoded) {
                Ok(_) => {
                    self.audio_frame(decoded).await;
                }
                Err(e) => {
                    drop(decoded);
                    debug!("{e:?}");
                    break;
                }
            }
        }
    }

    fn clear(&mut self) {
        self.decoder.flush();
        let n = self.packet_receiver.len();
        if n > 0 {
            for _ in 0..n {
                if let Ok(p) = self.packet_receiver.try_recv() {
                    drop(p);
                }
            }
        }
    }

    fn end(self) {
        drop(self.sender);
        drop(self.packet_receiver);
    }

    async fn consume(mut self) {
        loop {
            tokio::select! {
                p = self.packet_receiver.recv() => {
                    if let Some(packet) = p {
                        self.packet(packet).await;
                    } else {
                        break;
                    }
                }
                r = self.sender.ctrl.recv() => {
                    match r {
                        Ok(o) => {
                            match o {
                                ImageCtrl::Seek(_) => {
                                    self.clear();
                                }
                                ImageCtrl::Pause(_) => {}
                                ImageCtrl::Close => {
                                    break;
                                }
                            }
                        },
                        Err(e) => {
                            debug!("{e}");
                        },
                    }
                }
            };
        }
        self.end();
    }
}

///MediaWrapper.
struct MediaWrapper {
    input: Input,
    map: HashMap<usize, MpscSender<Packet>>,
}

impl MediaWrapper {
    fn new(p: &str) -> Result<Self> {
        input(p)
            .map_err(|e| to_err(ErrorKind::Media, e))
            .map(|input| Self {
                input,
                map: HashMap::new(),
            })
    }

    async fn decode(&mut self, sender: ImageDataSender) {
        let mut ctrl = sender.ctrl.resubscribe();
        let mut sender = Some(sender);
        for stream in self.input.streams() {
            let stream_index = stream.index();
            match Context::from_parameters(stream.parameters()) {
                Ok(context) => {
                    let decoder = context.decoder();
                    match decoder.medium() {
                        MediaType::Video => match decoder.video() {
                            Ok(video_decoder) => {
                                debug!("video_stream_index: {stream_index}");
                                if let Some(sender) = sender.take() {
                                    let (p_sender, p_receiver) = mpsc_channel(VIDEO_PACKET_SIZE);
                                    self.map.insert(stream_index, p_sender);
                                    let handle = Handle::current();
                                    std::thread::spawn(move || {
                                        handle.block_on(async {
                                            let o = VideoConsumer::new(
                                                video_decoder,
                                                p_receiver,
                                                sender,
                                            );
                                            o.consume().await;
                                        });
                                    });
                                } else {
                                    warn!("other video stream");
                                }
                            }
                            Err(e) => {
                                error!("video: {e:?}");
                            }
                        },
                        MediaType::Audio => match decoder.audio() {
                            Ok(audio_decoder) => {
                                debug!("audio_stream_index: {stream_index}");
                                let (p_sender, p_receiver) = mpsc_channel(AUDIO_PACKET_SIZE);
                                self.map.insert(stream_index, p_sender);
                                let ctrl = ctrl.resubscribe();
                                tokio::spawn(async move {
                                    let o = AudioConsumer::new(audio_decoder, p_receiver, ctrl);
                                    o.consume().await;
                                });
                            }
                            Err(e) => {
                                error!("audio: {e:?}");
                            }
                        },
                        _ => {
                            trace!("other_stream_index: {stream_index}");
                        }
                    }
                }
                Err(e) => {
                    warn!("from_parameters: {e:?}");
                }
            };
        }

        let f = unsafe { self.input.as_mut_ptr() };

        info!("start packet");
        for (stream, packet) in self.input.packets() {
            let stream_index = stream.index();
            if let Some(packet_sender) = self.map.get_mut(&stream_index) {
                if let Err(e) = packet_sender.send(packet).await {
                    debug!("{e:?}");
                }
            }
            if let Ok(o) = ctrl.try_recv() {
                match o {
                    ImageCtrl::Seek(n) => {
                        seek(f, &stream, n as i32);
                    }
                    ImageCtrl::Pause(_) => {}
                    ImageCtrl::Close => {
                        break;
                    }
                }
            }
        }

        for (stream_index, _) in self.map.drain() {
            trace!("stream_index: {stream_index} end");
        }
    }
}

fn seek(f: *mut AVFormatContext, stream: &Stream, pos: i32) {
    let tar = (pos * AV_TIME_BASE).rescale(TIME_BASE, stream.time_base());
    unsafe {
        av_seek_frame(f, stream.index() as i32, tar, AVSEEK_FLAG_BACKWARD);
    }
}

pub(crate) fn build(s: String) -> ImageDataReceiver {
    let (sender, receiver) = image_build_channel();
    std::thread::spawn(move || {
        if let Err(e) = Runtime::new().map(|rt| {
            rt.block_on(async {
                match MediaWrapper::new(&s) {
                    Ok(mut w) => {
                        w.decode(sender).await;
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
