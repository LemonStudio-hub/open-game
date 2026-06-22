use crate::components::GameState;

/// Snapshot of input state, populated once per frame.
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub jump_pressed: bool,
    pub shoot_down: bool,
    pub start_pressed: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            jump_pressed: false,
            shoot_down: false,
            start_pressed: false,
        }
    }
}

/// Current game state and transition timers.
pub struct GameStateRes {
    pub state: GameState,
    pub title_pulse: f32,
    pub game_over_timer: f32,
}

impl Default for GameStateRes {
    fn default() -> Self {
        Self {
            state: GameState::Title,
            title_pulse: 0.0,
            game_over_timer: 0.0,
        }
    }
}

/// Score tracking.
pub struct ScoreRes {
    pub score: i32,
    pub high_score: i32,
}

impl Default for ScoreRes {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
        }
    }
}

/// Player lives.
pub struct LivesRes {
    pub lives: i32,
}

impl Default for LivesRes {
    fn default() -> Self {
        Self {
            lives: crate::MAX_LIVES,
        }
    }
}

/// Camera state.
pub struct CameraRes {
    pub camera_x: f32,
    pub camera_y: f32,
    pub shake_amount: f32,
}

impl Default for CameraRes {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            shake_amount: 0.0,
        }
    }
}

/// Enemy spawn timing and difficulty.
pub struct SpawnRes {
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub difficulty_timer: f32,
}

impl Default for SpawnRes {
    fn default() -> Self {
        Self {
            spawn_timer: 1.5,
            spawn_interval: 2.0,
            difficulty_timer: 0.0,
        }
    }
}
