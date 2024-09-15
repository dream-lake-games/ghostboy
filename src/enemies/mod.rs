use crate::prelude::*;

pub mod skelly;
pub use skelly::*;

pub(super) struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        skelly::register_skelly(app);
    }
}
