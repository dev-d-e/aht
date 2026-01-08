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
        PCM::new("default", Direction::Playback, false)
            .map(|pcm| {
                match HwParams::any(&pcm) {
                    Ok(hwp) => {
                        let _ = hwp.set_channels(p.channels);
                        let _ = hwp.set_rate(p.rate, ValueOr::Nearest);
                        let _ = hwp.set_format(sample_to_format(p.format));
                        let _ = hwp.set_access(AlsaAccess::RWInterleaved);
                        let _ = pcm.hw_params(&hwp);
                    }
                    Err(e) => {
                        error!("HwParams Err: {:?}", e);
                    }
                }
                Self { pcm }
            })
            .ok()
    }

    pub(super) async fn playback(&mut self, mut receiver: SoundDataReceiver) {
        let io = self.pcm.io_bytes();
        loop {
            tokio::select! {
                o = receiver.data.recv() => {
                    if let Some(buf) = o {
                        if let Err(e) = io.writei(buf.bytes()) {
                            debug!("{e}");
                        }
                        drop(buf);
                    } else {
                        break;
                    }
                }
                r = receiver.ctrl.recv() => {
                    match r {
                        Ok(o) => {
                            match o{
                                ImageCtrl::Seek(_)  => {},
                                ImageCtrl::Pause(o) => {
                                    let _ = self.pcm.pause(o);
                                },
                                ImageCtrl::Close => {
                                    let _ = self.pcm.drain();
                                    return ;
                                },
                            }
                        },
                        Err(e) => {
                            debug!("{e}");
                        },
                    }
                }
            }
        }
        if self.pcm.state() != State::Running {
            let _ = self.pcm.start();
        };
        let _ = self.pcm.drain();
    }
}
