use crate::prelude::*;

mod my_ldtk_maint;

pub use my_ldtk_maint::MyLdtkWaitForGTran;

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .register_ldtk_entity_for_layer::<TestEntBundle>("Entities", "GBoy");

        my_ldtk_maint::register_ldtk_maint(app);
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct TestEntBundle {
    name: Name,
    pos: MyLdtkWaitForGTran,
    static_rx: StaticRx,
    anim: AnimMan<GBoyAnim>,
}
impl Default for TestEntBundle {
    fn default() -> Self {
        Self {
            name: Name::new("TestEnt"),
            pos: MyLdtkWaitForGTran::dyno(default()),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                Hbox::new().with_offset(0.0, -1.0).with_size(8, 12),
            ),
            anim: AnimMan::new(),
        }
    }
}
