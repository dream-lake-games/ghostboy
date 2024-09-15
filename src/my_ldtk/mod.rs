use crate::prelude::*;

mod my_ldtk_levels;
mod my_ldtk_maint;

pub use my_ldtk_levels::CurrentLevelHelpers;
pub use my_ldtk_maint::MyLdtkWait;

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell_for_layer::<WallBundle>("Ground", 1)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Platform", 1)
            .register_ldtk_entity_for_layer::<GBoyBundle>("Entities", "GBoy");

        my_ldtk_levels::register_my_ldtk_levels(app);
        my_ldtk_maint::register_ldtk_maint(app);
    }
}
