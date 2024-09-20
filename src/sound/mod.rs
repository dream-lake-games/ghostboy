use crate::prelude::*;

pub mod effect;
pub mod music;

pub use effect::*;
pub use music::*;

#[derive(Debug, Resource)]
pub struct SoundSettings {
    pub main_volume: f32,
    pub effect_volume: f32,
    pub music_volume: f32,
}
impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            main_volume: 0.6,
            effect_volume: 0.6,
            music_volume: 0.6,
        }
    }
}

pub(super) struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundSettings::default());
        app.add_plugins(MusicPlugin);
        app.add_plugins(SoundEffectPlugin);
    }
}
