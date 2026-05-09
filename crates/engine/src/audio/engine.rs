use std::collections::HashMap;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode};

use super::source::AudioHandle;

pub struct AudioEngine {
    context: AudioContext,
    master_gain: GainNode,
    buffers: HashMap<u32, AudioBuffer>,
    next_id: u32,
    sources: Vec<PlayingSource>,
    music_gain: GainNode,
    sfx_gain: GainNode,
}

struct PlayingSource {
    _source: AudioBufferSourceNode,
    _gain: GainNode,
    _handle: AudioHandle,
}

impl AudioEngine {
    pub fn new() -> Result<Self, String> {
        let context = AudioContext::new().map_err(|e| format!("Failed to create AudioContext: {:?}", e))?;

        let master_gain = context.create_gain().map_err(|e| format!("Failed to create master gain: {:?}", e))?;
        let _ = master_gain.connect_with_audio_node(&context.destination());

        let music_gain = context.create_gain().map_err(|e| format!("Failed to create music gain: {:?}", e))?;
        let _ = music_gain.connect_with_audio_node(&master_gain);

        let sfx_gain = context.create_gain().map_err(|e| format!("Failed to create sfx gain: {:?}", e))?;
        let _ = sfx_gain.connect_with_audio_node(&master_gain);

        Ok(Self {
            context,
            master_gain,
            buffers: HashMap::new(),
            next_id: 0,
            sources: Vec::new(),
            music_gain,
            sfx_gain,
        })
    }

    pub fn add_buffer(&mut self, buffer: AudioBuffer) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.buffers.insert(id, buffer);
        id
    }

    pub fn play(&mut self, buffer_id: u32, volume: f32, looping: bool) -> Option<AudioHandle> {
        let buffer = self.buffers.get(&buffer_id)?;

        let source = self.context.create_buffer_source().ok()?;
        source.set_buffer(Some(buffer));
        source.set_loop(looping);

        let gain = self.context.create_gain().ok()?;
        gain.gain().set_value(volume);
        let _ = source.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.sfx_gain);

        source.start().ok()?;

        let handle = AudioHandle(buffer_id);
        self.sources.push(PlayingSource {
            _source: source,
            _gain: gain,
            _handle: handle,
        });

        Some(handle)
    }

    pub fn play_music(&mut self, buffer_id: u32, volume: f32) -> Option<AudioHandle> {
        let buffer = self.buffers.get(&buffer_id)?;

        let source = self.context.create_buffer_source().ok()?;
        source.set_buffer(Some(buffer));
        source.set_loop(true);

        let gain = self.context.create_gain().ok()?;
        gain.gain().set_value(volume);
        let _ = source.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&self.music_gain);

        source.start().ok()?;

        let handle = AudioHandle(buffer_id);
        self.sources.push(PlayingSource {
            _source: source,
            _gain: gain,
            _handle: handle,
        });

        Some(handle)
    }

    pub fn set_master_volume(&self, volume: f32) {
        self.master_gain.gain().set_value(volume.max(0.0));
    }

    pub fn set_music_volume(&self, volume: f32) {
        self.music_gain.gain().set_value(volume.max(0.0));
    }

    pub fn set_sfx_volume(&self, volume: f32) {
        self.sfx_gain.gain().set_value(volume.max(0.0));
    }

    pub fn resume(&self) {
        let _ = self.context.resume();
    }

    pub fn suspend(&self) {
        let _ = self.context.suspend();
    }

    pub fn context(&self) -> &AudioContext {
        &self.context
    }

    pub fn is_resumed(&self) -> bool {
        self.context.state() == web_sys::AudioContextState::Running
    }

}
