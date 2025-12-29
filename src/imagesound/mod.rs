use crate::error::*;
use crate::markup::*;
use crate::utils::*;
use bytes::Bytes;
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
use ffmpeg_next::{Rescale, Stream};
use getset::{Getters, MutGetters};
use skia_safe::{images, ColorType, Data, ISize, Image, ImageInfo, SamplingOptions, Size};
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::JoinHandle;

const BOUND_SIZE: usize = 8;
const CTRL_SIZE: usize = 1;
const IMAGE_DATA_FPS: f32 = 24.0;

#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug)]
struct ImageData {
    width: f32,
    height: f32,
    bytes: Bytes,
}

impl ImageData {
    fn new(width: f32, height: f32, bytes: Bytes) -> Self {
        Self {
            width,
            height,
            bytes,
        }
    }

    fn from_slice(width: f32, height: f32, buf: &[u8]) -> Self {
        Self::new(width, height, Bytes::copy_from_slice(buf))
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
        unsafe { Data::new_bytes(&self.bytes) }
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

enum ImageCtrl {
    Seek(i64),
    Pause(bool),
}

#[derive(Debug)]
struct ImageDataSender {
    data: SyncSender<ImageData>,
    ctrl: Receiver<ImageCtrl>,
}

impl ImageDataSender {
    fn new(data: SyncSender<ImageData>, ctrl: Receiver<ImageCtrl>) -> Self {
        Self { data, ctrl }
    }

    fn send(&self, o: ImageData) {
        let _ = self.data.send(o);
    }

    fn ctrl(&self) -> Option<ImageCtrl> {
        self.ctrl.try_recv().ok()
    }

    fn ctrl_block(&self) -> Option<ImageCtrl> {
        self.ctrl.recv().ok()
    }
}

#[derive(Debug, Getters, MutGetters)]
pub(crate) struct ImageDataReceiver {
    data: Receiver<ImageData>,
    #[getset(get = "pub(crate)")]
    buffer: Option<Image>,
    ctrl: SyncSender<ImageCtrl>,
    fps_ctrl: FpsCtrl,
}

impl ImageDataReceiver {
    fn new(data: Receiver<ImageData>, ctrl: SyncSender<ImageCtrl>) -> Self {
        Self {
            data,
            buffer: None,
            ctrl,
            fps_ctrl: FpsCtrl::new(IMAGE_DATA_FPS),
        }
    }

    fn build_channel() -> (ImageDataSender, Self) {
        let (sender, receiver) = sync_channel(BOUND_SIZE);
        let (sender2, receiver2) = sync_channel(CTRL_SIZE);
        (
            ImageDataSender::new(sender, receiver2),
            Self::new(receiver, sender2),
        )
    }

    pub(crate) fn data(&mut self, image_info: ImageInfo, r: &RectSide) -> &Option<Image> {
        if self.fps_ctrl.is_next() {
            if let Ok(data) = self.data.try_recv() {
                if let Some(i) = data.to_image(image_info, r) {
                    let o = self.buffer.replace(i);
                    drop(o);
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

    pub(crate) fn ctrl_seek(&mut self, o: i64) -> bool {
        self.ctrl(ImageCtrl::Seek(o))
    }

    pub(crate) fn ctrl_pause(&mut self, o: bool) -> bool {
        self.ctrl(ImageCtrl::Pause(o))
    }
}

struct VideoConsumer {
    decoder: Video,
    sws_context: Option<SwsContext>,
}

impl VideoConsumer {
    fn new(decoder: Video) -> Self {
        let mut sws_context = None;
        match decoder.converter(Pixel::RGBA) {
            Ok(s) => {
                sws_context.replace(s);
            }
            Err(e) => {
                error!("{e}");
            }
        }
        Self {
            decoder,
            sws_context,
        }
    }

    fn video_frame(&mut self, video_frame: &mut VideoFrame, sender: &ImageDataSender) {
        let timestamp = video_frame.timestamp();
        video_frame.set_pts(timestamp);

        if let Some(s) = &mut self.sws_context {
            let mut out = VideoFrame::empty();
            let _ = s.run(video_frame, &mut out);
            let dat = out.data(0);
            if dat.len() > 0 {
                sender.send(ImageData::from_slice(
                    out.width() as f32,
                    out.height() as f32,
                    dat,
                ));
            }
        }
    }

    fn end(self) {}
}

#[derive(Debug)]
struct SoundData {
    bytes: Bytes,
}

impl SoundData {
    fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    fn from_slice(buf: &[u8]) -> Self {
        Self::new(Bytes::copy_from_slice(buf))
    }
}

impl From<Box<[u8]>> for SoundData {
    fn from(o: Box<[u8]>) -> Self {
        Self::new(Bytes::from(o))
    }
}

impl From<Vec<u8>> for SoundData {
    fn from(o: Vec<u8>) -> Self {
        Self::new(Bytes::from(o))
    }
}

enum SoundDataWrapper {
    Data(SoundData),
    Pause(bool),
}

#[derive(Debug)]
struct SoundDataSender {
    data: SyncSender<SoundDataWrapper>,
}

impl SoundDataSender {
    fn new(data: SyncSender<SoundDataWrapper>) -> Self {
        Self { data }
    }

    fn send(&self, o: SoundData) {
        let _ = self.data.send(SoundDataWrapper::Data(o));
    }

    fn ctrl_pause(&self, o: bool) {
        let _ = self.data.send(SoundDataWrapper::Pause(o));
    }
}

#[derive(Debug)]
struct SoundDataReceiver {
    data: Receiver<SoundDataWrapper>,
}

impl SoundDataReceiver {
    fn new(data: Receiver<SoundDataWrapper>) -> Self {
        Self { data }
    }

    fn build_channel() -> (SoundDataSender, Self) {
        let (sender, receiver) = sync_channel(BOUND_SIZE);
        (SoundDataSender::new(sender), Self::new(receiver))
    }
}

struct AudioConsumer {
    decoder: Audio,
    swr_context: Option<SwrContext>,
    sender: SoundDataSender,
    sound_thread: JoinHandle<()>,
}

impl AudioConsumer {
    fn new(decoder: Audio) -> Self {
        let (sender, receiver) = SoundDataReceiver::build_channel();

        let p = SoundProperty::new(&decoder);

        let swr_context = decoder
            .resampler(p.format, decoder.channel_layout(), p.rate)
            .inspect_err(|e| error!("swr_context: {e}"))
            .ok();

        let sound_thread = std::thread::spawn(|| {
            #[cfg(target_os = "linux")]
            if let Some(mut sound) = linux::Sound::new(p) {
                sound.playback(receiver);
            }
            #[cfg(not(any(target_os = "linux", target_os = "ios", target_os = "android")))]
            while let Ok(data) = receiver.data.try_recv() {
                drop(data);
            }
        });

        Self {
            decoder,
            swr_context,
            sender,
            sound_thread,
        }
    }

    fn audio_frame(&mut self, audio_frame: &mut AudioFrame) {
        if audio_frame.is_planar() {
            if let Some(s) = &mut self.swr_context {
                let mut out = AudioFrame::empty();
                let _ = s.run(audio_frame, &mut out);
                let dat = out.data(0);
                if dat.len() > 0 {
                    self.sender.send(SoundData::from_slice(dat));
                }
            }
        } else {
            let dat = audio_frame.data(0);
            if dat.len() > 0 {
                self.sender.send(SoundData::from_slice(dat));
            }
        }
    }

    fn end(self) {
        drop(self.sender);
        let _ = self.sound_thread.join();
    }
}

enum DecoderType {
    Video(VideoConsumer),
    Audio(AudioConsumer),
    None,
}

///MediaWrapper.
struct MediaWrapper {
    input: Input,
    map: HashMap<usize, DecoderType>,
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

    fn decode(&mut self, sender: ImageDataSender) {
        for stream in self.input.streams() {
            let stream_index = stream.index();
            match Context::from_parameters(stream.parameters()) {
                Ok(context) => {
                    let decoder = context.decoder();
                    match decoder.medium() {
                        MediaType::Video => match decoder.video() {
                            Ok(video_decoder) => {
                                debug!("video_stream_index: {:?}", stream_index);
                                let d = DecoderType::Video(VideoConsumer::new(video_decoder));
                                self.map.insert(stream_index, d);
                            }
                            Err(e) => {
                                error!("video: {:?}", e);
                            }
                        },
                        MediaType::Audio => match decoder.audio() {
                            Ok(audio_decoder) => {
                                debug!("audio_stream_index: {:?}", stream_index);
                                let d = DecoderType::Audio(AudioConsumer::new(audio_decoder));
                                self.map.insert(stream_index, d);
                            }
                            Err(e) => {
                                error!("audio: {:?}", e);
                            }
                        },
                        _ => {
                            trace!("other_stream_index: {:?}", stream_index);
                        }
                    }
                }
                Err(e) => {
                    warn!("from_parameters: {:?}", e);
                }
            };
        }

        let f = unsafe { self.input.as_mut_ptr() };

        for (stream, packet) in self.input.packets() {
            let stream_index = stream.index();
            if let Some(decoder_type) = self.map.get_mut(&stream_index) {
                match decoder_type {
                    DecoderType::Video(consumer) => {
                        decode_ctrl(
                            &sender,
                            f,
                            &stream,
                            || {
                                consumer.decoder.flush();
                            },
                            |_| {},
                        );

                        if let Err(e) = consumer.decoder.send_packet(&packet) {
                            error!("send_packet: {:?}", e);
                            continue;
                        }
                        let mut decoded = VideoFrame::empty();
                        while consumer.decoder.receive_frame(&mut decoded).is_ok() {
                            consumer.video_frame(&mut decoded, &sender);
                        }
                        drop(decoded);
                    }
                    DecoderType::Audio(consumer) => {
                        decode_ctrl(
                            &sender,
                            f,
                            &stream,
                            || {
                                consumer.decoder.flush();
                            },
                            |n| {
                                consumer.sender.ctrl_pause(n);
                            },
                        );

                        if let Err(e) = consumer.decoder.send_packet(&packet) {
                            error!("send_packet: {:?}", e);
                            continue;
                        }
                        let mut decoded = AudioFrame::empty();
                        while consumer.decoder.receive_frame(&mut decoded).is_ok() {
                            consumer.audio_frame(&mut decoded);
                        }
                        drop(decoded);
                    }
                    DecoderType::None => {}
                }
            }
        }

        for (stream_index, decoder_type) in self.map.drain() {
            match decoder_type {
                DecoderType::Video(consumer) => consumer.end(),
                DecoderType::Audio(consumer) => consumer.end(),
                DecoderType::None => {}
            }
            trace!("stream_index: {:?} end", stream_index);
        }
    }
}

fn seek(f: *mut AVFormatContext, stream: &Stream, pos: i64) {
    let tar = (pos * AV_TIME_BASE as i64).rescale(TIME_BASE, stream.time_base());
    unsafe {
        av_seek_frame(f, stream.index() as i32, tar, AVSEEK_FLAG_BACKWARD);
    }
}

fn decode_ctrl(
    sender: &ImageDataSender,
    f: *mut AVFormatContext,
    stream: &Stream,
    mut flush: impl FnMut(),
    mut audio_pause: impl FnMut(bool),
) {
    if let Some(c) = sender.ctrl() {
        match c {
            ImageCtrl::Seek(n) => {
                seek(f, stream, n);
                flush();
            }
            ImageCtrl::Pause(n) => {
                audio_pause(n);
                if n {
                    while let Some(c) = sender.ctrl_block() {
                        match c {
                            ImageCtrl::Seek(n) => {
                                seek(f, stream, n);
                                flush();
                            }
                            ImageCtrl::Pause(n) => {
                                if !n {
                                    audio_pause(n);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn build(s: String) -> ImageDataReceiver {
    let (sender, receiver) = ImageDataReceiver::build_channel();
    std::thread::spawn(move || {
        if let Ok(mut w) = MediaWrapper::new(&s) {
            w.decode(sender);
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
