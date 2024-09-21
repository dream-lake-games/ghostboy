use crate::prelude::*;

pub mod arrow;
pub mod guillo;
pub mod parallax;
pub mod wallish;

pub use arrow::*;
pub use parallax::*;
pub use wallish::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        arrow::register_arrows(app);
        guillo::register_guillo(app);
        parallax::register_parallax(app);
        wallish::register_wallish(app);
    }
}
