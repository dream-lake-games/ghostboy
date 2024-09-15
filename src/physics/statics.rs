use crate::prelude::*;

use super::physics_maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Copy, Debug, Reflect)]
pub enum StaticTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
}
#[derive(Clone, Copy, Debug, Reflect)]
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
    pub fn new(data: Vec<(StaticTxKind, Hbox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: StaticTxKind, hbox: Hbox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticTxComp {
    pub kind: StaticTxKind,
    pub ctrl: Entity,
    pub hbox: Hbox,
}
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticTxCtrl {
    pub comps: Vec<Entity>,
    pub colls: Vec<CollKey>,
}
impl_physics_comp!(StaticTxKind, StaticTxComp, StaticTxCtrl);

#[derive(Bundle)]
pub struct StaticRx {
    ctrl: StaticRxCtrl,
    comps: PhysicsComps<StaticRxComp>,
}
impl StaticRx {
    pub fn new(data: Vec<(StaticRxKind, Hbox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: StaticRxKind, hbox: Hbox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticRxComp {
    pub kind: StaticRxKind,
    pub ctrl: Entity,
    pub hbox: Hbox,
}
impl_physics_comp!(StaticRxKind, StaticRxComp, StaticRxCtrl);
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticRxCtrl {
    pub comps: Vec<Entity>,
    pub colls: Vec<CollKey>,
}
