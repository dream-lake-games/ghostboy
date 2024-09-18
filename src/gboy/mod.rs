use crate::prelude::*;

mod control;
pub mod death;
mod gboy_fsm;
pub mod spawn;

pub use death::GBoyDying;
pub use spawn::TombstoneActive;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct GBoy;

#[derive(Bundle)]
struct GBoyBundle {
    name: Name,
    marker: GBoy,
    anim: AnimMan<GBoyAnim>,
    static_rx: StaticRx,
    static_rx_touches: StaticRxTouches,
    trigger_tx: TriggerTx,
    trigger_rx: TriggerRx,
    pos: Pos,
    dyno: Dyno,
    gravity: Gravity,
    dyno_facing: DynoFacing,
    spatial: SpatialBundle,
}
impl GBoyBundle {
    fn new(pos: Pos) -> Self {
        let hbox = Hbox::new().with_size(9, 12).with_offset(0.0, -1.0);
        Self {
            name: Name::new("gboy"),
            marker: default(),
            anim: AnimMan::new(),
            static_rx: StaticRx::single(StaticRxKind::Default, hbox.clone()),
            static_rx_touches: default(),
            trigger_tx: TriggerTx::single(TriggerTxKind::GBoy, hbox.clone()),
            trigger_rx: TriggerRx::single(TriggerRxKind::GBoy, hbox.clone()),
            pos,
            dyno: default(),
            gravity: default(),
            dyno_facing: default(),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.as_vec2().extend(20.0),
            )),
        }
    }
}

pub fn maintain_sanity(gboy: Query<Entity, With<GBoy>>) {
    assert!(gboy.iter().count() == 1);
}

pub(super) struct GBoyPlugin;
impl Plugin for GBoyPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, GBoy);

        app.add_systems(
            Update,
            maintain_sanity.run_if(in_state(LevelState::Playing)),
        );

        control::register_control(app);
        gboy_fsm::register_gboy_fsm(app);
        death::register_death(app);
        spawn::register_spawn(app);
    }
}
