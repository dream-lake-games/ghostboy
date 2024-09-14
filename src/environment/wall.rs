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
            wait: MyLdtkWaitForGTran::no_dyno(),
            wall: Wall,
            static_tx: StaticTx::single(StaticTxKind::Solid, Hbox::new().with_size(16, 16)),
        }
    }
}
