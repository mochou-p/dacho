// dacho/core/app/src/timer.rs

use std::time::Instant;

use dacho_log::create_log;
use dacho_components::TimeComponent;


pub struct Timer {
    start_time: Instant,
    last_time:  Instant
}

impl Timer {
    pub fn new() -> Self {
        create_log!(debug);

        let start_time = Instant::now();
        let last_time  = start_time;

        Self { start_time, last_time }
    }

    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn get_component(&mut self) -> TimeComponent {
        TimeComponent { delta: self.delta() }
    }

    fn delta(&mut self) -> f32 {
        let delta = self.last_time.elapsed().as_secs_f32();

        self.last_time = Instant::now();

        delta
    }
}

