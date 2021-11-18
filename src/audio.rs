use crate::prelude::*;

use macroquad::audio::{self, PlaySoundParams};

// This is used to determine what volume to use when playing a sound file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VolumeCategory {
    SoundEffect,
    Music,
}

#[derive(Copy, Clone)]
pub struct Sound {
    sound: audio::Sound,
    pub category: VolumeCategory,
}

impl Sound {
    pub fn play(&self, should_loop: bool) {
        let params = PlaySoundParams {
            looped: should_loop,
            volume: get_volume(self.category),
        };

        audio::play_sound(self.sound, params);
    }
}

pub fn get_volume(category: VolumeCategory) -> f32 {
    let config = storage::get::<Config>();
    let master_volume = config.master_volume as f32 / 100.0;
    match category {
        VolumeCategory::SoundEffect => (config.sound_effects_volume as f32 / 100.0) * master_volume,
        VolumeCategory::Music => (config.music_volume as f32 / 100.0) * master_volume,
    }
}

pub async fn load_sound_from_bytes(category: VolumeCategory, bytes: &[u8]) -> Result<Sound> {
    let sound = audio::load_sound_from_bytes(bytes).await?;

    let res = Sound { sound, category };

    Ok(res)
}

pub async fn load_sound<P: AsRef<Path>>(category: VolumeCategory, path: P) -> Result<Sound> {
    let path = path.as_ref();
    let bytes = load_file(path).await?;

    load_sound_from_bytes(category, &bytes).await
}

pub fn play_sound(sound: Sound, should_loop: bool) {
    sound.play(should_loop);
}
