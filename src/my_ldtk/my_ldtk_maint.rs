use crate::prelude::*;

/// Exists because when the spatial gets added to bundles from thingys after the custom component is added
#[derive(Component, Default)]
pub struct MyLdtkWaitForGTran {
    dyno: Option<Dyno>,
}
impl MyLdtkWaitForGTran {
    pub fn no_dyno() -> Self {
        Self::default()
    }

    pub fn dyno(dyno: Dyno) -> Self {
        Self { dyno: Some(dyno) }
    }
}

fn bless_with_pos_n_dyno(
    mut commands: Commands,
    ents: Query<(Entity, &GlobalTransform, &MyLdtkWaitForGTran)>,
) {
    for (eid, gtran, wait) in &ents {
        commands.entity(eid).remove::<MyLdtkWaitForGTran>();
        commands
            .entity(eid)
            .insert(Pos::new(gtran.translation().x, gtran.translation().y));
        if let Some(dyno) = wait.dyno.clone() {
            commands.entity(eid).insert(dyno);
        }
    }
}

pub(super) fn register_ldtk_maint(app: &mut App) {
    app.add_systems(PreUpdate, bless_with_pos_n_dyno);
}
