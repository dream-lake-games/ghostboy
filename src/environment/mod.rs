use crate::prelude::*;

pub mod wall;

pub use wall::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, _app: &mut App) {}
}
