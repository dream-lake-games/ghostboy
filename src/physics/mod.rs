use crate::prelude::*;

pub mod bullet_time;
pub mod dyno;
pub mod hbox;
pub mod physics_maint;
pub mod pos;
pub mod statics;
pub mod triggers;

pub use bullet_time::*;
pub use dyno::*;
pub use hbox::*;
pub use pos::*;
pub use statics::*;
pub use triggers::*;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

/// The physics-internal set that resolves collisions
/// NOTE: Subset of PhysicsSet
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CollisionSet;

/// The physics-internal set that resolves collisions
/// NOTE: Subset of CollisionSet
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PosSet;

pub(super) struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(
            app,
            // bullet_time.rs
            // dyno.rs
            BulletTime,
            // hbox.rs
            Hbox,
            // maint.rs
            // pos.rs
            Pos,
            IPos,
            // statics.rs
            StaticRxKind,
            StaticRxComp,
            StaticRxCtrl,
            StaticTxKind,
            StaticTxComp,
            StaticTxCtrl,
            // triggers.rs
            TriggerRxKind,
            TriggerRxComp,
            TriggerRxCtrl,
            TriggerTxKind,
            TriggerTxComp,
            TriggerTxCtrl,
        );

        app.add_plugins(bullet_time::BulletTimePlugin);

        pos::register_pos(app);
    }
}
