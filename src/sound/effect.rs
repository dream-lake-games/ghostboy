use crate::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

use super::SoundSettings;

#[derive(Component, Debug, Clone)]
pub struct SoundEffect {
    path: String,
    base_volume: f32,
}
impl Default for SoundEffect {
    fn default() -> Self {
        Self {
            path: "".into(),
            base_volume: 1.0,
        }
    }
}
impl SoundEffect {
    pub fn universal(path: &str, base_volume: f32) -> SoundEffect {
        Self {
            path: path.into(),
            base_volume,
        }
    }
}

/// Attach to sound effects that you want to guarantee are not playing rn
/// Will avoid duplicates by looking at the path
#[derive(Component)]
pub struct NoDupSound;

/// Used internally
#[derive(Component)]
struct NoDupInternal(String);

fn spawn_sound_effects(
    mut commands: Commands,
    no_dups: Query<&NoDupInternal>,
    lacking: Query<(Entity, &SoundEffect, Option<&NoDupSound>), Without<PlaybackSettings>>,
    sound_root: Res<SoundRoot>,
    asset_server: Res<AssetServer>,
) {
    let mut no_dup_set = HashSet::new();
    for no_dup in &no_dups {
        no_dup_set.insert(no_dup.0.clone());
    }
    for (eid, effect, optional_no_dup) in lacking.iter() {
        if optional_no_dup.is_some() && no_dup_set.contains(&effect.path) {
            continue;
        }
        commands
            .spawn((
                Name::new("sound_effect"),
                effect.clone(),
                AudioBundle {
                    source: asset_server.load(&effect.path),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(0.0),
                        ..default()
                    },
                },
            ))
            .set_parent(sound_root.eid());
        commands.entity(eid).despawn_recursive();
    }
}

fn update_sound_effect_volume(
    mut sounds: Query<(&SoundEffect, &mut PlaybackSettings, &mut AudioSink)>,
    sound_settings: Res<SoundSettings>,
) {
    for (effect, mut playback, sink) in sounds.iter_mut() {
        playback.volume = Volume::new(
            sound_settings.main_volume * sound_settings.effect_volume * effect.base_volume,
        );
        sink.set_volume(playback.volume.abs());
    }
}

pub(super) struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_sound_effects, update_sound_effect_volume));
    }
}
