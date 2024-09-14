use crate::prelude::*;

/// Exists because when the spatial gets added to bundles from thingys after the custom component is added
#[derive(Component, Default)]
pub struct MyLdtkWaitForGTran;

fn bless_with_pos(
    mut commands: Commands,
    ents: Query<(Entity, &GlobalTransform), With<MyLdtkWaitForGTran>>,
) {
    for (eid, gtran) in &ents {
        commands.entity(eid).remove::<MyLdtkWaitForGTran>();
        commands
            .entity(eid)
            .insert(Pos::new(gtran.translation().x, gtran.translation().y));
    }
}

pub(super) fn register_ldtk_maint(app: &mut App) {
    app.add_systems(PreUpdate, bless_with_pos);
}
