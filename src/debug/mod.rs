use crate::prelude::*;

mod draw_hitboxes;

fn debug_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/play.ldtk"),
        transform: Transform::from_translation(Vec3::new(-80.0, -72.0, 0.0)),
        ..Default::default()
    });
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = MainLayer::render_layers();
}

fn debug_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut force: Query<&mut Dyno, With<StaticRxCtrl>>,
) {
    // Horizontal movement
    let mut hor_dir = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        hor_dir = -1.0;
    } else if keyboard.pressed(KeyCode::KeyD) {
        hor_dir = 1.0;
    }
    let hor_mag = 60.0;
    for mut dyno in &mut force {
        dyno.vel.x = hor_dir * hor_mag;
    }
    // Jump
    let jump_mag = 100.0;
    for mut dyno in &mut force {
        if keyboard.just_pressed(KeyCode::KeyW) {
            dyno.vel.y = jump_mag;
        }
        dyno.vel.y -= 0.4;
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

        draw_hitboxes::register_draw_hitboxes(app);
    }
}
