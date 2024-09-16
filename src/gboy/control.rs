use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct GBoyControlConsts {
    max_hor_speed: f32,
    hor_acc: f32,
    air_hor_friction: f32,
    max_ver_speed: f32,
    slowdown_acc: f32,
    jump_vel: f32,
    dash_speed: f32,
    dash_time: f32,
}
impl Default for GBoyControlConsts {
    fn default() -> Self {
        Self {
            max_hor_speed: 80.0,
            hor_acc: 480.0,
            air_hor_friction: 0.66,
            max_ver_speed: 169.0,
            slowdown_acc: 960.0,
            jump_vel: 156.0,
            dash_speed: 200.0,
            dash_time: 0.1,
        }
    }
}

#[derive(Clone, Debug, Reflect)]
struct Dashing {
    dir: Vec2,
    time_left: f32,
}
impl Dashing {
    fn new(dir: Vec2, time_left: f32) -> Self {
        Self { dir, time_left }
    }
}
impl Component for Dashing {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            world.commands().entity(eid).remove::<Gravity>();
        });
        hooks.on_remove(|mut world, eid, _| {
            world.commands().entity(eid).insert(Gravity::default());
        });
    }
}

/// Control horizontal movement
/// NOTE: Only when gboy is NOT dashing
fn control_gboy_hor(
    dir4_input: Res<Dir4Input>,
    mut gboy_q: Query<(&mut Dyno, &StaticRxTouches), (With<GBoy>, Without<Dashing>)>,
    consts: Res<GBoyControlConsts>,
) {
    let Ok((mut dyno, touches)) = gboy_q.get_single_mut() else {
        return;
    };
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
}

/// Control vertical movemnt
/// NOTE: Only when gboy is NOT dashing
fn control_gboy_ver(
    gbutton_input: Res<GButtonInput>,
    mut gboy_q: Query<(&mut Dyno, &StaticRxTouches), (With<GBoy>, Without<Dashing>)>,
    consts: Res<GBoyControlConsts>,
) {
    let Ok((mut dyno, touches)) = gboy_q.get_single_mut() else {
        return;
    };
    if gbutton_input.just_pressed(GButton::A) && touches[Dir4::Down] {
        dyno.vel.y = consts.jump_vel;
    }
}

/// Limit speed
/// NOTE: Only when gboy is NOT dashing
fn limit_speed(
    mut gboy_q: Query<&mut Dyno, (With<GBoy>, Without<Dashing>)>,
    consts: Res<GBoyControlConsts>,
) {
    let Ok(mut dyno) = gboy_q.get_single_mut() else {
        return;
    };
    let acc = consts.slowdown_acc / FRAMERATE;
    // Hor
    if dyno.vel.x.abs() > consts.max_hor_speed {
        let adjust = dyno.vel.x.signum() * -1.0 * acc;
        dyno.vel.x += adjust;
        if dyno.vel.x.abs() < consts.max_hor_speed {
            // Don't slow past limit
            dyno.vel.x = dyno.vel.x.signum() * consts.max_hor_speed;
        }
    }
    // Ver
    if dyno.vel.y.abs() > consts.max_ver_speed {
        let adjust = dyno.vel.y.signum() * -1.0 * acc;
        dyno.vel.y += adjust;
        if dyno.vel.y.abs() < consts.max_hor_speed {
            // Don't slow past limit
            dyno.vel.y = dyno.vel.y.signum() * consts.max_hor_speed;
        }
    }
}

/// Potentially start a dash
/// NOTE: Only when gboy is NOT dashing
fn start_gboy_dash(
    dir4_input: Res<Dir4Input>,
    gbutton_input: Res<GButtonInput>,
    mut gboy_q: Query<(Entity, &mut Dyno, &Facing), (With<GBoy>, Without<Dashing>)>,
    consts: Res<GBoyControlConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut dyno, facing)) = gboy_q.get_single_mut() else {
        return;
    };
    if gbutton_input.just_pressed(GButton::B) {
        let mut vel = dir4_input.as_vec2();
        if vel == Vec2::ZERO {
            vel = if facing.right() { Vec2::X } else { -Vec2::X };
        }
        vel = vel.normalize_or_zero() * consts.dash_speed;
        dyno.vel = vel;
        commands
            .entity(eid)
            .insert(Dashing::new(vel, consts.dash_time));
    }
}

/// Update (read: end) gboy mid dash
fn end_gboy_dash(
    mut commands: Commands,
    mut gboy_q: Query<(Entity, &mut Dashing), With<GBoy>>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((eid, mut dash)) = gboy_q.get_single_mut() else {
        return;
    };
    dash.time_left -= bullet_time.delta_seconds();
    if dash.time_left <= 0.0 {
        commands.entity(eid).remove::<Dashing>();
    }
}

pub(super) fn register_control(app: &mut App) {
    app.insert_resource(GBoyControlConsts::default());
    debug_resource!(app, GBoyControlConsts);

    app.add_systems(
        BulletUpdate,
        (control_gboy_hor, limit_speed)
            .chain()
            .after(PhysicsSet)
            .run_if(one_gboy_exists),
    );
    app.add_systems(
        Update,
        (control_gboy_ver, start_gboy_dash, end_gboy_dash)
            .after(PhysicsSet)
            .run_if(one_gboy_exists),
    );
}
