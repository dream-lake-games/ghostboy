use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct Dyno {
    pub vel: Vec2,
}

#[derive(Clone, Debug, Reflect, Default)]
pub struct DynoFacing;
const DYNO_FACING_MIN_VEL: f32 = 0.01;

fn update_dyno_facing(mut ents: Query<(&mut Facing, &Dyno), With<DynoFacing>>) {
    for (mut facing, dyno) in &mut ents {
        if dyno.vel.x.abs() > DYNO_FACING_MIN_VEL {
            *facing = Facing::from_f32(dyno.vel.x);
        }
    }
}
impl Component for DynoFacing {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            if world.get::<Facing>(eid).is_none() {
                world.commands().entity(eid).insert(Facing::Right);
            }
        });
    }
}

#[derive(Component, Clone, Debug, Reflect)]
// Marks components that should be affected by gravity
pub struct Gravity {
    pub mult: f32,
}
impl Default for Gravity {
    fn default() -> Self {
        Self { mult: 1.0 }
    }
}

pub(super) fn register_dynos(app: &mut App) {
    app.add_systems(
        Update,
        update_dyno_facing
            .in_set(PhysicsSet)
            .after(super::CollSet)
            .run_if(in_state(PhysicsState::Active)),
    );
}
