#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioHandle(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioCategory {
    Music,
    Sfx,
}

pub struct AudioSource {
    pub buffer_id: u32,
    pub volume: f32,
    pub looping: bool,
    pub category: AudioCategory,
}

impl AudioSource {
    pub fn new(buffer_id: u32) -> Self {
        Self {
            buffer_id,
            volume: 1.0,
            looping: false,
            category: AudioCategory::Sfx,
        }
    }

    pub fn music(buffer_id: u32) -> Self {
        Self {
            buffer_id,
            volume: 1.0,
            looping: true,
            category: AudioCategory::Music,
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }
}
