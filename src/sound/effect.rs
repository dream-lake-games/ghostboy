use crate::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

#[derive(Component, Clone, Debug, Reflect)]
pub struct SoundMult(pub f32);

#[derive(Component, Clone, Debug, Reflect)]
pub enum OneSound {
    Replace,
    Ignore,
}

fn spawn_sound_effects(
    mut commands: Commands,
    existing: Query<(Entity, &SoundEffect), With<PlaybackSettings>>,
    new: Query<
        (Entity, &SoundEffect, Option<&SoundMult>, Option<&OneSound>),
        Without<PlaybackSettings>,
    >,
    sound_root: Res<SoundRoot>,
    asset_server: Res<AssetServer>,
    sound_mults: Res<SoundMults>,
) {
    let mut exist_map: HashMap<SoundEffect, Vec<Entity>> = default();
    for (eid, se) in &existing {
        match exist_map.get_mut(se) {
            Some(ptr) => ptr.push(eid),
            None => {
                exist_map.insert(se.clone(), vec![eid]);
            }
        };
    }
    for (eid, se, omult, oone) in &new {
        let surviving = match oone {
            Some(OneSound::Replace) => {
                for eid in exist_map.get(se).unwrap_or(&vec![]) {
                    commands.entity(*eid).despawn_recursive();
                }
                true
            }
            Some(OneSound::Ignore) => exist_map.get(se) == None,
            None => true,
        };
        if surviving {
            let mult = omult.map(|s| s.0).unwrap_or(1.0) * *sound_mults.map.get(se).unwrap_or(&1.0);
            commands
                .entity(eid)
                .insert(AudioBundle {
                    source: asset_server.load(se.path()),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new(mult),
                        ..default()
                    },
                })
                .set_parent(sound_root.eid());
        } else {
            commands.entity(eid).despawn_recursive();
        }
    }
}

pub(super) struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_sound_effects);
    }
}
