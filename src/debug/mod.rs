use crate::prelude::*;

mod draw_hitboxes;

fn debug_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/world.ldtk"),
        ..default()
    });
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = MainLayer::render_layers();
}

fn debug_update() {}

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugSet;

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup.in_set(DebugSet));
        app.add_systems(Update, debug_update.in_set(DebugSet).after(PhysicsSet));

        draw_hitboxes::register_draw_hitboxes(app);
    }
}
