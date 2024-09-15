use crate::prelude::*;

pub mod wall;

pub use wall::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>("Ground", 1)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Platform", 1);
    }
}
