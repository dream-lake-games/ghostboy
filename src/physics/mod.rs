use crate::prelude::*;

pub mod bullet_time;
pub mod hbox;
pub mod maint;
pub mod statics;
pub mod triggers;

pub use bullet_time::*;
pub use hbox::*;
pub use statics::*;
pub use triggers::*;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

pub(super) struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(
            app,
            // bullet_time.rs
            BulletTime,
            // hbox.rs
            Hbox,
            // maint.rs
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
    }
}
