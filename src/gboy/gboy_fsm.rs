use crate::prelude::*;

/// This function is going to be a BEAST and probably very ugly.
/// But that's actually good. The alternative is to allow the gboy animation
/// to be updated from multiple systems. Debugging this would be a nightmare.
/// Although the logic here will get hairy, at least it's all in one place
/// when things go wrong.
fn update_gboy_animation(mut gboy: Query<(&mut AnimMan<GBoyAnim>, &Dyno, &StaticRxTouches)>) {
    let (mut anim, dyno, srx_touches) = gboy.single_mut();
    let moving_hor = dyno.vel.x.abs() > 0.1;

    if moving_hor {
        anim.set_flip_x(dyno.vel.x < 0.0);
    }

    if srx_touches[Dir::Down] {
        if moving_hor {
            anim.set_state(GBoyAnim::Run);
        } else {
            anim.set_state(GBoyAnim::Stand);
        }
    } else {
        anim.set_state(GBoyAnim::Air);
    }
}

pub(super) fn register_gboy_fsm(app: &mut App) {
    app.add_systems(
        Update,
        update_gboy_animation
            .after(PhysicsSet)
            .run_if(one_gboy_exists),
    );
}
