use crate::prelude::*;

use super::physics_maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
    /// A platform that only stops things that are moving down
    PassUp,
}
#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticRxKind {
    /// Pushes the rx ctrl out of tx comps, sets vel to zero along plane of intersection
    Default,
}

/// When alongside a StaticRxCtrl, it will update every frame how long ago since a collision in that dir
/// NOTE: Only counts a collision being "pushed" into
#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct StaticRxTouches {
    map: HashMap<Dir4, f32>,
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
    pub coll_keys: Vec<CollKey>,
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
    pub coll_keys: Vec<CollKey>,
}

impl std::ops::Index<Dir4> for StaticRxTouches {
    type Output = f32;

    fn index(&self, index: Dir4) -> &Self::Output {
        self.map.get(&index).unwrap_or(&0.0)
    }
}
impl StaticRxTouches {
    pub fn set(&mut self, dir: Dir4, val: f32) {
        self.map.insert(dir, val);
    }
}
