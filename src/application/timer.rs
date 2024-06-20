// dacho/src/application/timer.rs

// std
use std::time::Instant;

// debug
#[cfg(debug_assertions)]
use {
    super::logger::Logger,
    crate::log
};

pub struct Timer {
    start_time:  Instant,
    #[cfg(debug_assertions)]
    last_time:   Instant,
    #[cfg(debug_assertions)]
    last_fps:    f32,
    #[cfg(debug_assertions)]
    rate:        usize,
    #[cfg(debug_assertions)]
    frames:      usize,
    #[cfg(debug_assertions)]
    first_frame: bool
}

impl Timer {
    pub fn new(
        #[cfg(debug_assertions)]
        rate: usize
    ) -> Self {
        #[cfg(debug_assertions)]
        log!(info, "Creating Timer");

        let start_time = Instant::now();

        #[cfg(debug_assertions)] {
            let last_time   = start_time;
            let last_fps    = 0.0;
            let frames      = 1;
            let first_frame = true;

            Self { start_time, last_time, last_fps, rate, frames, first_frame }
        }

        #[cfg(not(debug_assertions))]
        Self { start_time }
    }

    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    #[cfg(debug_assertions)]
    pub fn fps(&mut self) {
        match self {
            Self { frames, rate, .. } if frames == rate => {
                let now        = Instant::now();
                let elapsed    = now.duration_since(self.last_time).as_secs_f32();
                self.last_time = now;
                self.frames    = 1;

                let fps = (self.rate as f32 / elapsed).round();

                if (fps - self.last_fps).abs() >= 1.0 {
                    log!(info_r, "{fps} FPS{}", " ".repeat(27));

                    self.last_fps = fps;
                }
            },
            _ => {
                if self.first_frame {
                    log!(info_r, "Waiting for first Timer tick");

                    self.first_frame = false;
                }

                self.frames += 1;
            }
        }
    }
}

