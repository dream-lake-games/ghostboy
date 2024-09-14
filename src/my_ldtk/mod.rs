use crate::prelude::*;

mod my_ldtk_maint;

pub use my_ldtk_maint::MyLdtkWaitForGTran;

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1);

        my_ldtk_maint::register_ldtk_maint(app);
    }
}
