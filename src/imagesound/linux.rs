use super::*;
use alsa::pcm::{Access as AlsaAccess, Format as AlsaFormat, HwParams, State};
use alsa::{Direction, PCM, ValueOr};

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

    pub(super) async fn playback(&mut self, mut s: SoundDataOutput) {
        let _ = self.pcm.prepare();
        let io = self.pcm.io_bytes();
        let mut interval = interval_millis();
        let mut pause = false;
        loop {
            tokio::select! {
                biased;
                r = s.pause_receiver.recv() => {
                    if let Ok(o) = r {
                        pause = o;
                        let _ = self.pcm.pause(o);
                        info!("Sound pause");
                    }
                }
                r = s.seek_receiver.recv() => {
                    if let Some(o) = r {
                        let _ = self.pcm.drop();
                        let _ = self.pcm.prepare();
                        while let Ok(buf) = s.data_receiver.try_recv(){
                            drop(buf);
                        }
                        s.v_data.clear();
                        o.wait().await;
                        info!("Sound seek");
                    }
                }
                r = s.close_receiver.recv() => {
                    if let Some(o) = r {
                        let _ = self.pcm.drop();
                        o.wait().await;
                        info!("Sound close");
                        return;
                    }
                }
                _ = interval.tick() => {}
                r = s.data_receiver.recv(), if s.v_data.len() < SOUND_DATA_SIZE => {
                    if let Some(buf) = r {
                        s.v_data.push_back(buf);
                    }
                }
            }
            if !pause {
                if let Some(mut buf) = s.v_data.pop_front() {
                    match io.writei(buf.bytes()) {
                        Ok(n) => {
                            let n = self.pcm.frames_to_bytes(n as i64) as usize;
                            if buf.has_data(n) {
                                s.v_data.push_front(buf);
                            } else {
                                drop(buf);
                            }
                        }
                        Err(e) => {
                            let _ = self.pcm.try_recover(e, false);
                        }
                    }
                }
                if self.pcm.state() != State::Running {
                    let _ = self.pcm.start();
                }
            }
        }
    }
}
