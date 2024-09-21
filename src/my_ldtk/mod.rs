use crate::prelude::*;

mod my_ldtk_levels;
mod my_ldtk_maint;

pub use my_ldtk_levels::CurrentLevelHelpers;
pub use my_ldtk_maint::{
    register_replaceable, LdtkDependents as MyLdtkDependents, MyLdtkReplacable, MyLdtkReplace,
    MyLdtkWait,
};

#[derive(Bundle, LdtkIntCell)]
struct HelpBundle {
    wait: MyLdtkWait,
}
impl Default for HelpBundle {
    fn default() -> Self {
        Self {
            wait: MyLdtkWait::parent_render_layers(MainLayer::render_layers()),
        }
    }
}

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
            .insert_resource(LevelSelection::iid("6707e010-4ce0-11ef-8458-1d8de6fabb3d"));
        my_ldtk_levels::register_my_ldtk_levels(app);
        my_ldtk_maint::register_ldtk_maint(app);
        for val in [1, 2, 3, 4] {
            app.register_ldtk_int_cell_for_layer::<HelpBundle>("Help", val);
        }
    }
}
