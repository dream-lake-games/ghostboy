use crate::prelude::*;

pub mod skelly;
pub use skelly::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySet;

pub(super) struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        skelly::register_skelly(app);
    }
}
