use crate::prelude::*;

mod draw_hitboxes;

fn debug_startup(mut gizmo_config_store: ResMut<GizmoConfigStore>) {
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = MainLayer::render_layers();
}

fn debug_update() {}

fn reload_level(
    keyboard: Res<ButtonInput<KeyCode>>,
    proj: Query<Entity, With<Handle<LdtkProject>>>,
    level_root: Res<LevelRoot>,
    mut commands: Commands,
    mut meta_state: ResMut<NextState<MetaState>>,
) {
    if keyboard.just_pressed(KeyCode::Backspace) {
        for ent in &proj {
            commands.entity(ent).despawn_recursive();
        }
        commands.entity(level_root.eid()).despawn_descendants();
        meta_state.set(
            LevelState::Loading(LevelLoadingState {
                world_path: "ldtk/world.ldtk".to_string(),
                level_iid: LevelIid::new("4f654670-73f0-11ef-b6ce-11a7b97a42a0"),
            })
            .to_meta_state(),
        );
    }
}

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugSet;

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup.in_set(DebugSet));
        app.add_systems(Update, debug_update.in_set(DebugSet).after(PhysicsSet));
        app.add_systems(Last, reload_level.run_if(in_state(LevelState::Playing)));

        draw_hitboxes::register_draw_hitboxes(app);
    }
}
