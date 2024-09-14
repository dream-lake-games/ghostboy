use crate::prelude::*;

use super::maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Debug, Reflect)]
pub enum StaticTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
}
#[derive(Clone, Debug, Reflect)]
pub enum StaticRxKind {
    /// Pushes the rx ctrl out of tx comps, sets vel to zero along plane of intersection
    Default,
}

// PLUMBING
#[derive(Bundle)]
pub struct StaticTx {
    ctrl: StaticTxCtrl,
    comps: PhysicsComps<StaticTxComp>,
}
impl StaticTx {
    pub fn new(data: Vec<(StaticTxKind, Hbox, Pos)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticTxComp {
    kind: StaticTxKind,
    ctrl: Entity,
    hbox: Hbox,
    offset: Pos,
}
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticTxCtrl {
    pub comps: Vec<Entity>,
}
impl_physics_comp!(StaticTxKind, StaticTxComp, StaticTxCtrl);

#[derive(Bundle)]
pub struct StaticRx {
    ctrl: StaticRxCtrl,
    comps: PhysicsComps<StaticRxComp>,
}
impl StaticRx {
    pub fn new(data: Vec<(StaticRxKind, Hbox, Pos)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticRxComp {
    kind: StaticRxKind,
    ctrl: Entity,
    hbox: Hbox,
    offset: Pos,
}
impl_physics_comp!(StaticRxKind, StaticRxComp, StaticRxCtrl);
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticRxCtrl {
    pub comps: Vec<Entity>,
}
