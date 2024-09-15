use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct Dyno {
    pub vel: Vec2,
}

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct DynoFacing;
const DYNO_FACING_MIN_VEL: f32 = 0.0001;

fn update_dyno_facing(mut ents: Query<(&mut Facing, &Dyno), With<DynoFacing>>) {
    for (mut facing, dyno) in &mut ents {
        if dyno.vel.x.abs() > DYNO_FACING_MIN_VEL {
            *facing = Facing::from_f32(dyno.vel.x);
        }
    }
}

pub(super) fn register_dynos(app: &mut App) {
    app.add_systems(
        Update,
        update_dyno_facing.in_set(PhysicsSet).after(super::CollSet),
    );
}
