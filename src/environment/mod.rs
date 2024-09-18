use crate::prelude::*;

pub mod arrow;
pub mod wallish;

pub use arrow::*;
pub use wallish::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>("Ground", 1)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Platform", 1);
        for val in [1, 2, 3, 4, 5, 6] {
            app.register_ldtk_int_cell_for_layer::<SpikeBundle>("Spikes", val);
        }

        arrow::register_arrows(app);
    }
}
