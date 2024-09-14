//! Pos functions as the source of truth for element translational placement.
//! It should be updated ONLY during `CollisionsSet`, which is a subset of `PhysicsSet`.
//! IPos is updated also in `CollisionsSet`, but is simply the rounded version of Pos.
//! Transforms are updated by looking at the IPos diffs, and adding.
//! This way we avoid global transform shenanigans.

use crate::prelude::*;

#[derive(Copy, Clone, Debug, Reflect)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}
impl Component for Pos {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let me = *world.get::<Self>(eid).expect("Couldn't get Pos after add");
            world.commands().entity(eid).insert(IPos::new(me));
        });
    }
}
impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct IPos {
    pub cur: IVec2,
    pub last: IVec2,
}
impl IPos {
    fn new(pos: Pos) -> Self {
        let rounded = pos.to_ivec();
        Self {
            cur: rounded,
            last: rounded,
        }
    }

    fn diff(&self) -> IVec2 {
        self.cur - self.last
    }
}

fn update_ipos(mut ents: Query<(&Pos, &mut IPos)>) {
    for (pos, mut ipos) in &mut ents {
        ipos.last = ipos.cur;
        ipos.cur = pos.to_ivec();
    }
}

fn update_transforms(mut ents: Query<(&IPos, &mut Transform)>) {
    for (ipos, mut tran) in &mut ents {
        let diff3 = ipos.diff().as_vec2().extend(0.0);
        tran.translation += diff3;
    }
}

pub(super) fn register_pos(app: &mut App) {
    app.add_systems(
        Update,
        (update_ipos, update_transforms)
            .chain()
            .in_set(PhysicsSet)
            .in_set(super::CollisionSet)
            .in_set(super::PosSet),
    );
}
