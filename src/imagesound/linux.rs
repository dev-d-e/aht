use super::*;
use alsa::pcm::{Access as AlsaAccess, Format as AlsaFormat, HwParams, State};
use alsa::{Direction, ValueOr, PCM};

fn sample_to_format(sample: Sample) -> AlsaFormat {
    match sample {
        Sample::U8(_) => AlsaFormat::U8,
        Sample::I16(_) => AlsaFormat::s16(),
        Sample::I32(_) => AlsaFormat::s32(),
        Sample::F32(_) => AlsaFormat::float(),
        Sample::I64(_) => AlsaFormat::float64(),
        Sample::F64(_) => AlsaFormat::float64(),
        Sample::None => AlsaFormat::Unknown,
    }
}

pub(super) struct Sound {
    pcm: PCM,
}

impl Sound {
    pub(super) fn new(p: SoundProperty) -> Option<Self> {
        if let Ok(pcm) = PCM::new("default", Direction::Playback, false) {
            match HwParams::any(&pcm) {
                Ok(hwp) => {
                    let _ = hwp.set_channels(p.channels);
                    let _ = hwp.set_rate(p.rate, ValueOr::Nearest);
                    let _ = hwp.set_format(sample_to_format(p.format));
                    let _ = hwp.set_access(AlsaAccess::RWInterleaved);
                    let _ = pcm.hw_params(&hwp);
                }
                Err(e) => {
                    println!("HwParams Err: {:?}", e);
                }
            }
            return Some(Self { pcm });
        }
        None
    }

    pub(super) fn playback(&mut self, receiver: SoundDataReceiver) {
        let io = self.pcm.io_bytes();
        while let Ok(w) = receiver.data.recv() {
            match w {
                SoundDataWrapper::Data(buf) => {
                    let _ = io.writei(&buf.bytes);
                }
                SoundDataWrapper::Pause(n) => {
                    let _ = self.pcm.pause(n);
                }
            }
        }
        if self.pcm.state() != State::Running {
            let _ = self.pcm.start();
        };
        let _ = self.pcm.drain();
    }
}
