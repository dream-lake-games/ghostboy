use crate::prelude::*;

mod gboy_fsm;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct GBoy;

#[derive(Bundle, LdtkEntity)]
pub struct GBoyBundle {
    name: Name,
    marker: GBoy,
    anim: AnimMan<GBoyAnim>,
    static_rx: StaticRx,
    static_rx_touches: StaticRxTouches,
    // TODO: Make spawner
    pos: MyLdtkWait,
}
impl Default for GBoyBundle {
    fn default() -> Self {
        Self {
            name: Name::new("gboy"),
            marker: default(),
            anim: AnimMan::new(),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                Hbox::new().with_offset(0.0, -1.0).with_size(8, 12),
            ),
            static_rx_touches: default(),
            pos: MyLdtkWait::dyno(default()),
        }
    }
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
        gboy_fsm::register_gboy_fsm(app);
    }
}
