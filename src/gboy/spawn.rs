use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
struct GBoySpawnPoint;

#[derive(Bundle, LdtkEntity)]
pub struct GBoySpawnPointBundle {
    name: Name,
    marker: GBoySpawnPoint,
    wait: MyLdtkWait,
    // Lol just to see the outline
    // trigger: TriggerTx,
}
impl Default for GBoySpawnPointBundle {
    fn default() -> Self {
        Self {
            name: Name::new("gboy_spawn_point"),
            marker: GBoySpawnPoint,
            wait: MyLdtkWait::default(),
            // trigger: TriggerTx::single(TriggerTxKind::Solid, Hbox::new().with_size(10, 10)),
        }
    }
}

fn simple_spawn(
    spawn_points: Query<&Pos, With<GBoySpawnPoint>>,
    mut commands: Commands,
    root: Res<LevelRoot>,
    current_helpers: Res<CurrentLevelHelpers>,
) {
    let Some(bounds) = current_helpers.bounds else {
        return;
    };
    let Some(pos) = spawn_points
        .iter()
        .filter(|pos| bounds.contains(pos.as_vec2()))
        .next()
    else {
        return;
    };
    commands
        .spawn(super::GBoyBundle::new(pos.clone()))
        .set_parent(root.eid());
}

pub(super) fn register_spawn(app: &mut App) {
    app.add_systems(PreUpdate, simple_spawn.run_if(no_gboy_exists));
}
