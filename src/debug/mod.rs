use crate::prelude::*;

fn play_startup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        AnimMan::<SuicidoBody>::new(),
        StaticTx::new(vec![]),
        StaticRx::new(vec![]),
    ));
}

fn play_update() {}

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugSet;

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_startup.in_set(DebugSet));
        app.add_systems(Update, play_update.in_set(DebugSet));
    }
}
