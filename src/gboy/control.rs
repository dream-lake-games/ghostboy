use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct GBoyControlConsts {
    max_hor_speed: f32,
    hor_acc: f32,
    air_hor_friction: f32,
    jump_vel: f32,
}
impl Default for GBoyControlConsts {
    fn default() -> Self {
        Self {
            max_hor_speed: 80.0,
            hor_acc: 480.0,
            air_hor_friction: 0.66,
            jump_vel: 156.0,
        }
    }
}

fn control_gboy_hor(
    dir4_input: Res<Dir4Input>,
    mut gboy_q: Query<(&mut Dyno, &StaticRxTouches)>,
    consts: Res<GBoyControlConsts>,
) {
    let (mut dyno, touches) = gboy_q.single_mut();
    let x = dir4_input.as_vec2().x;
    let acc = consts.hor_acc
        * if touches[Dir4::Down] {
            1.0
        } else {
            consts.air_hor_friction
        }
        / FRAMERATE;

    if x == 0.0 {
        if dyno.vel.x == 0.0 {
            return;
        }
        let adjust = dyno.vel.x.signum() * -1.0 * acc;
        if adjust.abs() > dyno.vel.x.abs() {
            dyno.vel.x = 0.0;
        } else {
            dyno.vel.x += adjust;
        }
    } else {
        dyno.vel.x += x.signum() * acc;
    }
    dyno.vel.x = dyno
        .vel
        .x
        .clamp(-consts.max_hor_speed, consts.max_hor_speed);
}

fn control_gboy_ver(
    gbutton_input: Res<GButtonInput>,
    mut gboy_q: Query<(&mut Dyno, &StaticRxTouches)>,
    consts: Res<GBoyControlConsts>,
) {
    let (mut dyno, touches) = gboy_q.single_mut();
    if gbutton_input.just_pressed(GButton::A) && touches[Dir4::Down] {
        dyno.vel.y = consts.jump_vel;
    }
}

pub(super) fn register_control(app: &mut App) {
    app.insert_resource(GBoyControlConsts::default());
    debug_resource!(app, GBoyControlConsts);

    app.add_systems(
        BulletUpdate,
        control_gboy_hor.after(PhysicsSet).run_if(one_gboy_exists),
    );
    app.add_systems(
        Update,
        control_gboy_ver.after(PhysicsSet).run_if(one_gboy_exists),
    );
}
