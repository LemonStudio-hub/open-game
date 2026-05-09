use web_sys::Performance;

#[derive(Debug)]
pub struct Time {
    performance: Performance,
    last_frame: f64,
    delta: f32,
    elapsed: f32,
    frame_count: u64,
    fps: f32,
    fps_timer: f64,
    fps_frame_count: u32,
    fixed_timestep: f32,
    time_scale: f32,
    accumulator: f32,
}

impl Time {
    pub fn new(performance: Performance) -> Self {
        Self {
            performance,
            last_frame: 0.0,
            delta: 0.0,
            elapsed: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_timer: 0.0,
            fps_frame_count: 0,
            fixed_timestep: 1.0 / 60.0,
            time_scale: 1.0,
            accumulator: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.last_frame = self.performance.now();
    }

    pub fn update(&mut self) {
        let now = self.performance.now();
        let raw_delta = (now - self.last_frame) / 1000.0;
        self.last_frame = now;

        self.delta = (raw_delta as f32 * self.time_scale).min(0.25);
        self.elapsed += self.delta;
        self.frame_count += 1;

        self.accumulator += self.delta;

        self.fps_frame_count += 1;
        self.fps_timer += raw_delta;
        if self.fps_timer >= 1.0 {
            self.fps = self.fps_frame_count as f32 / self.fps_timer as f32;
            self.fps_frame_count = 0;
            self.fps_timer = 0.0;
        }
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn fixed_timestep(&self) -> f32 {
        self.fixed_timestep
    }

    pub fn set_fixed_timestep(&mut self, timestep: f32) {
        self.fixed_timestep = timestep;
    }

    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    pub fn accumulator(&self) -> f32 {
        self.accumulator
    }

    pub fn consume_fixed_step(&mut self) -> bool {
        if self.accumulator >= self.fixed_timestep {
            self.accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }

    pub fn alpha(&self) -> f32 {
        self.accumulator / self.fixed_timestep
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    duration: f32,
    elapsed: f32,
    repeating: bool,
    finished: bool,
}

impl Timer {
    pub fn once(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            repeating: false,
            finished: false,
        }
    }

    pub fn repeating(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            repeating: true,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.finished && !self.repeating {
            return false;
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed -= self.duration;
            if !self.repeating {
                self.finished = true;
            }
            return true;
        }
        false
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).min(1.0)
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn remaining(&self) -> f32 {
        (self.duration - self.elapsed).max(0.0)
    }
}
