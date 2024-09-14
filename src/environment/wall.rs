use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    wait: MyLdtkWaitForGTran,
    wall: Wall,
    static_tx: StaticTx,
}
impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wait: MyLdtkWaitForGTran,
            wall: Wall,
            static_tx: StaticTx::single(StaticTxKind::Solid, Hbox::new(16, 16), IVec2::ZERO),
        }
    }
}
