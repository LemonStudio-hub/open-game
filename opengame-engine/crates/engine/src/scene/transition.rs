use crate::color::Color;

pub enum TransitionType {
    FadeIn,
    FadeOut,
    FadeInOut,
}

pub struct Transition {
    transition_type: TransitionType,
    duration: f32,
    elapsed: f32,
    color: Color,
    finished: bool,
}

impl Transition {
    pub fn fade_in(duration: f32, color: Color) -> Self {
        Self {
            transition_type: TransitionType::FadeIn,
            duration,
            elapsed: 0.0,
            color,
            finished: false,
        }
    }

    pub fn fade_out(duration: f32, color: Color) -> Self {
        Self {
            transition_type: TransitionType::FadeOut,
            duration,
            elapsed: 0.0,
            color,
            finished: false,
        }
    }

    pub fn fade_in_out(duration: f32, color: Color) -> Self {
        Self {
            transition_type: TransitionType::FadeInOut,
            duration,
            elapsed: 0.0,
            color,
            finished: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.finished = true;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).min(1.0)
    }

    pub fn alpha(&self) -> f32 {
        let progress = self.progress();
        match self.transition_type {
            TransitionType::FadeIn => 1.0 - progress,
            TransitionType::FadeOut => progress,
            TransitionType::FadeInOut => {
                if progress < 0.5 {
                    progress * 2.0
                } else {
                    1.0 - (progress - 0.5) * 2.0
                }
            }
        }
    }

    pub fn color(&self) -> Color {
        self.color.with_alpha(self.alpha())
    }
}
