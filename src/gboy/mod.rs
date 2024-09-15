use crate::prelude::*;

mod gboy_fsm;
mod spawn;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct GBoy;

#[derive(Bundle)]
struct GBoyBundle {
    name: Name,
    marker: GBoy,
    anim: AnimMan<GBoyAnim>,
    static_rx: StaticRx,
    static_rx_touches: StaticRxTouches,
    pos: Pos,
    facing: Facing,
    dyno: Dyno,
    dyno_facing: DynoFacing,
    spatial: SpatialBundle,
}
impl GBoyBundle {
    fn new(pos: Pos) -> Self {
        Self {
            name: Name::new("gboy"),
            marker: default(),
            anim: AnimMan::new(),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                Hbox::new().with_offset(0.0, -1.0).with_size(9, 12),
            ),
            static_rx_touches: default(),
            pos,
            facing: Facing::Right,
            dyno: default(),
            dyno_facing: default(),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.as_vec2().extend(20.0),
            )),
        }
    }
}

pub fn no_gboy_exists(ents: Query<Entity, With<GBoy>>) -> bool {
    ents.is_empty()
}

pub fn one_gboy_exists(ents: Query<Entity, (With<GBoy>, With<Dyno>)>) -> bool {
    if ents.is_empty() {
        return false;
    }
    if ents.iter().count() > 1 {
        warn!("Too many gboys, something bad is happening");
        return false;
    }
    true
}

pub(super) struct GBoyPlugin;
impl Plugin for GBoyPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, GBoy);

        gboy_fsm::register_gboy_fsm(app);
        spawn::register_spawn(app);
    }
}
