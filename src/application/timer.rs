// dacho/src/application/timer.rs

pub struct Timer {
    start_time:  std::time::Instant,
    #[cfg(debug_assertions)]
    last_time:   std::time::Instant,
    #[cfg(debug_assertions)]
    last_fps:    usize,
    #[cfg(debug_assertions)]
    rate:        usize,
    #[cfg(debug_assertions)]
    frames:      usize,
    #[cfg(debug_assertions)]
    first_frame: bool
}

#[cfg(debug_assertions)]
use super::logger::Logger;

impl Timer {
    pub fn new(
        #[cfg(debug_assertions)]
        rate: usize
    ) -> Self {
        #[cfg(debug_assertions)]
        Logger::info("Creating Timer");

        let start_time = std::time::Instant::now();

        #[cfg(debug_assertions)]
        {
            let last_time   = start_time;
            let last_fps    = 0;
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
        if self.frames == self.rate {
            let now        = std::time::Instant::now();
            let elapsed    = now.duration_since(self.last_time).as_secs_f32();
            self.last_time = now;
            self.frames    = 1;

            let fps = (self.rate as f32 / elapsed).round() as usize;

            if fps != self.last_fps {
                Logger::info_r(format!("{} FPS                           ", fps));

                self.last_fps = fps;
            }
        } else {
            if self.first_frame {
                Logger::info_r("Waiting for first Timer tickrate");

                self.first_frame = false;
            }

            self.frames += 1
        }
    }
}

