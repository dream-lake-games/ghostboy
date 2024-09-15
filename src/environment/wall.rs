use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    wait: MyLdtkWait,
    pos: Pos,
    wall: Wall,
    static_tx: StaticTx,
}
impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wait: MyLdtkWait::parent_render_layers(MainLayer::render_layers()),
            pos: Pos::new(-6000.0, -6000.0), // Will be overwritten
            wall: Wall,
            static_tx: StaticTx::single(StaticTxKind::Solid, Hbox::new().with_size(8, 8)),
        }
    }
}
