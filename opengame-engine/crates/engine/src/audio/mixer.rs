use super::engine::AudioEngine;

pub struct Mixer {
    master_volume: f32,
    music_volume: f32,
    sfx_volume: f32,
    muted: bool,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            muted: false,
        }
    }

    pub fn set_master_volume(&mut self, volume: f32, engine: &AudioEngine) {
        self.master_volume = volume.max(0.0);
        if !self.muted {
            engine.set_master_volume(self.master_volume);
        }
    }

    pub fn set_music_volume(&mut self, volume: f32, engine: &AudioEngine) {
        self.music_volume = volume.max(0.0);
        if !self.muted {
            engine.set_music_volume(self.music_volume);
        }
    }

    pub fn set_sfx_volume(&mut self, volume: f32, engine: &AudioEngine) {
        self.sfx_volume = volume.max(0.0);
        if !self.muted {
            engine.set_sfx_volume(self.sfx_volume);
        }
    }

    pub fn mute(&mut self, engine: &AudioEngine) {
        self.muted = true;
        engine.set_master_volume(0.0);
    }

    pub fn unmute(&mut self, engine: &AudioEngine) {
        self.muted = false;
        engine.set_master_volume(self.master_volume);
    }

    pub fn toggle_mute(&mut self, engine: &AudioEngine) {
        if self.muted {
            self.unmute(engine);
        } else {
            self.mute(engine);
        }
    }

    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
    }
}
