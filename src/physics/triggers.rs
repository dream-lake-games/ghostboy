use crate::prelude::*;

use super::physics_maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum TriggerTxKind {
    GBoy,
    Arrow,
    Spike,
    Dummy,
}
#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum TriggerRxKind {
    GBoy,
    Tombstone,
}

// PLUMBING
#[derive(Bundle, Reflect, Debug, Clone)]
pub struct TriggerTx {
    ctrl: TriggerTxCtrl,
    comps: PhysicsComps<TriggerTxComp>,
}
impl TriggerTx {
    pub fn new(data: Vec<(TriggerTxKind, Hbox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: TriggerTxKind, hbox: Hbox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerTxComp {
    pub kind: TriggerTxKind,
    pub ctrl: Entity,
    pub hbox: Hbox,
}
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TriggerTxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
impl_physics_comp!(TriggerTxKind, TriggerTxComp, TriggerTxCtrl);

#[derive(Bundle)]
pub struct TriggerRx {
    ctrl: TriggerRxCtrl,
    comps: PhysicsComps<TriggerRxComp>,
}
impl TriggerRx {
    pub fn new(data: Vec<(TriggerRxKind, Hbox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: TriggerRxKind, hbox: Hbox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerRxComp {
    pub kind: TriggerRxKind,
    pub ctrl: Entity,
    pub hbox: Hbox,
}
impl_physics_comp!(TriggerRxKind, TriggerRxComp, TriggerRxCtrl);
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TriggerRxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
