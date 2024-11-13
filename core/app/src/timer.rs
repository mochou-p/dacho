// dacho/core/app/src/timer.rs

use std::time::Instant;

use {
    dacho_log::create_log,
    dacho_components::TimeComponent
};


pub struct Timer {
    start_time: Instant,
    _last_time:  Instant
}

impl Timer {
    pub fn new() -> Self {
        create_log!(debug);

        let start_time = Instant::now();
        let _last_time = start_time;

        Self { start_time, _last_time }
    }

    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn _get_component(&mut self) -> TimeComponent {
        TimeComponent { delta: self._delta() }
    }

    fn _delta(&mut self) -> f32 {
        let delta = self._last_time.elapsed().as_secs_f32();

        self._last_time = Instant::now();

        delta
    }
}

