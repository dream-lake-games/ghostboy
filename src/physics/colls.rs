use crate::prelude::*;

pub type CollKey = u32;

#[derive(Debug, Clone, Reflect)]
pub struct StaticCollRec {
    pub pos: Pos,
    pub push: Vec2,
    /// Before collision, component of receivers velocity in collision normal direction
    pub rx_perp: Vec2,
    /// Before collision, component of receivers velocity perpendicular to normal direction
    /// Name is weird because it's "parallel" to original vel of rx
    pub rx_par: Vec2,
    pub rx_ctrl: Entity,
    pub rx_kind: StaticRxKind,
    pub tx_ctrl: Entity,
    pub tx_kind: StaticTxKind,
}
#[derive(Resource, Debug, Reflect)]
pub struct StaticColls {
    map: HashMap<CollKey, StaticCollRec>,
}
impl StaticColls {
    pub fn insert(&mut self, key: CollKey, rec: StaticCollRec) {
        self.map.insert(key, rec);
    }
    pub fn get(&self, key: &CollKey) -> Option<&StaticCollRec> {
        self.map.get(key)
    }
    pub fn get_refs(&self, coll_keys: &[CollKey]) -> Vec<&StaticCollRec> {
        coll_keys.iter().filter_map(|key| self.get(key)).collect()
    }
    pub fn all(&self) -> Vec<&StaticCollRec> {
        self.map.values().into_iter().collect()
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct TriggerCollRec {
    pub pos: Pos,
    pub rx_ctrl: Entity,
    pub rx_kind: TriggerRxKind,
    pub tx_ctrl: Entity,
    pub tx_kind: TriggerTxKind,
}
#[derive(Resource, Debug, Reflect)]
pub struct TriggerColls {
    map: HashMap<CollKey, TriggerCollRec>,
}
impl TriggerColls {
    pub fn insert(&mut self, key: CollKey, rec: TriggerCollRec) {
        self.map.insert(key, rec);
    }
    pub fn get(&self, key: &CollKey) -> Option<&TriggerCollRec> {
        self.map.get(key)
    }
    pub fn get_refs(&self, coll_keys: &[CollKey]) -> Vec<&TriggerCollRec> {
        coll_keys.iter().filter_map(|key| self.get(key)).collect()
    }
}

fn reset_colls_every_frame(
    mut static_colls: ResMut<StaticColls>,
    mut trigger_colls: ResMut<TriggerColls>,
    mut srx_ctrls: Query<&mut StaticRxCtrl>,
    mut stx_ctrls: Query<&mut StaticTxCtrl>,
    mut trx_ctrls: Query<&mut TriggerRxCtrl>,
    mut ttx_ctrls: Query<&mut TriggerTxCtrl>,
) {
    // Eh at some point we may want to shrink memory used, but this probably fine
    static_colls.map.clear();
    trigger_colls.map.clear();
    macro_rules! clear_coll_keys {
        ($thing:expr) => {
            for mut thing in &mut $thing {
                thing.coll_keys.clear();
            }
        };
    }
    clear_coll_keys!(srx_ctrls);
    clear_coll_keys!(stx_ctrls);
    clear_coll_keys!(trx_ctrls);
    clear_coll_keys!(ttx_ctrls);
}

pub(super) fn register_colls(app: &mut App) {
    app.insert_resource(StaticColls { map: default() });
    app.insert_resource(TriggerColls { map: default() });

    app.add_systems(First, reset_colls_every_frame.in_set(PhysicsSet));
    // debug_resource!(app, StaticColls);
    // debug_resource!(app, TriggerColls);
}
