use crate::prelude::*;

pub mod bullet_time;

pub use bullet_time::*;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

pub(super) struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bullet_time::BulletTimePlugin);
    }
}
