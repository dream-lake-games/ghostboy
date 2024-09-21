use crate::prelude::*;

use super::control::CanDash;

/// This function is going to be a BEAST and probably very ugly.
/// But that's actually good. The alternative is to allow the gboy animation
/// to be updated from multiple systems. Debugging this would be a nightmare.
/// Although the logic here will get hairy, at least it's all in one place
/// when things go wrong.
fn update_gboy_animation(
    mut gboy: Query<(
        &mut AnimMan<GBoyAnim>,
        &AnimBodyProgress<GBoyAnim>,
        &Dyno,
        &StaticRxTouches,
        Option<&CanDash>,
    )>,
    mut commands: Commands,
) {
    let (mut anim, animp, dyno, srx_touches, can_dash) = gboy.single_mut();
    let moving_hor = dyno.vel.x.abs() > 0.1;

    if srx_touches[Dir4::Down] == 0.0 {
        if moving_hor {
            anim.set_state(GBoyAnim::Run);
            if animp.get_body_ix(AnimBody_GBoyAnim::run) == Some(1) {
                commands.spawn((vec![SoundEffect::Normal2].pick(), SoundMult(0.06)));
            }
        } else {
            anim.set_state(GBoyAnim::Stand);
        }
    } else {
        if can_dash.is_some() {
            anim.set_state(GBoyAnim::AirFull);
        } else {
            anim.set_state(GBoyAnim::AirEmpty);
        }
    }
}

pub(super) fn register_gboy_fsm(app: &mut App) {
    app.add_systems(
        Update,
        update_gboy_animation
            .after(PhysicsSet)
            .run_if(in_state(LevelState::Playing)),
    );
}
