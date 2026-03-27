use std::time::{Instant, Duration};

pub struct Timer {
    ticks_per_second: f32,
    adjust_time: f32,
    time_scale: f32,
    passed_time: f32,
    last_instant: Instant,
    last_time: f32,
}

impl Timer {
    pub fn new(ticks_per_second: f32) -> Self {
        let now = Instant::now();
        Self {
            ticks_per_second,
            adjust_time: 1.0,
            time_scale: 1.0,
            passed_time: 0.0,
            last_instant: now,
            last_time: now.elapsed().as_secs_f32(),
        }
    }

    pub fn advance_time(&mut self) {
        let now = Instant::now();
        let passed_ms = now.duration_since(self.last_instant).as_millis() as f64;

        if passed_ms > 1000.0 {
            // Adjust time logic
            let adjust_time_t = passed_ms / 1000.0; // simplified
            self.adjust_time += ((adjust_time_t as f32) - self.adjust_time) * 0.2;
            self.last_instant = now;
        }

        if passed_ms < 0.0 {
            self.last_instant = now;
        }

        let now_secs = now.elapsed().as_secs_f32();
        let mut passed_seconds = (now_secs - self.last_time) * self.adjust_time;
        self.last_time = now_secs;

        if passed_seconds < 0.0 {
            passed_seconds = 0.0;
        }
        if passed_seconds > 1.0 {
            passed_seconds = 1.0;
        }

        self.passed_time += passed_seconds * self.time_scale * self.ticks_per_second;
    }

    pub fn get_passed_time(&self) -> f32 {
        self.passed_time
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }
}