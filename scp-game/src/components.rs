/// Game state enum (used in GameStateRes).
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Title,
    Playing,
    GameOver,
}

/// Player component — attached to the single player entity.
/// Position/velocity stored here (not in a shared Pos component) because
/// the engine's QueryDoubleMut requires A and B to have different TypeIds.
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub facing_right: bool,
    pub invincible: f32,
    pub flash: f32,
    pub shoot_timer: f32,
}

/// Enemy component.
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub hp: i32,
    pub alive: bool,
    pub on_ground: bool,
    pub shoot_timer: f32,
    pub ai_timer: f32,
    pub flash: f32,
    pub size: f32,
}

/// Bullet component.
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
    pub is_player: bool,
}

/// Particle component.
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
    pub color_idx: u8,
}
