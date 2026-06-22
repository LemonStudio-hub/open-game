/// Game state enum.
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Title,
    Playing,
    GameOver,
}

/// Marker component for the ground entity.
pub struct Ground;

/// Player component — position synced via Transform2D.
pub struct Player {
    pub facing_right: bool,
    pub invincible: f32,
    pub flash: f32,
    pub shoot_timer: f32,
    pub on_ground: bool,
}

/// Enemy component — position synced via Transform2D.
pub struct Enemy {
    pub hp: i32,
    pub alive: bool,
    pub on_ground: bool,
    pub shoot_timer: f32,
    pub ai_timer: f32,
    pub flash: f32,
    pub size: f32,
}

/// Bullet component — NOT in physics system, positions updated manually.
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
    pub is_player: bool,
}

/// Particle component — purely visual, no physics.
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
