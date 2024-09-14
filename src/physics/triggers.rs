use crate::prelude::*;

use super::maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Debug, Reflect)]
pub enum TriggerTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
}
#[derive(Clone, Debug, Reflect)]
pub enum TriggerRxKind {
    /// Pushes the rx ctrl out of tx comps, sets vel to zero along plane of intersection
    Default,
}

// PLUMBING
#[derive(Bundle)]
pub struct TriggerTx {
    ctrl: TriggerTxCtrl,
    comps: PhysicsComps<TriggerTxComp>,
}
impl TriggerTx {
    pub fn new(data: Vec<(TriggerTxKind, Hbox, Pos)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerTxComp {
    kind: TriggerTxKind,
    ctrl: Entity,
    hbox: Hbox,
    offset: Pos,
}
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TriggerTxCtrl {
    pub comps: Vec<Entity>,
}
impl_physics_comp!(TriggerTxKind, TriggerTxComp, TriggerTxCtrl);

#[derive(Bundle)]
pub struct TriggerRx {
    ctrl: TriggerRxCtrl,
    comps: PhysicsComps<TriggerRxComp>,
}
impl TriggerRx {
    pub fn new(data: Vec<(TriggerRxKind, Hbox, Pos)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerRxComp {
    kind: TriggerRxKind,
    ctrl: Entity,
    hbox: Hbox,
    offset: Pos,
}
impl_physics_comp!(TriggerRxKind, TriggerRxComp, TriggerRxCtrl);
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct TriggerRxCtrl {
    pub comps: Vec<Entity>,
}
