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

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct Spike;

#[derive(Bundle, LdtkIntCell)]
pub struct SpikeBundle {
    wait: MyLdtkWait,
    pos: Pos,
    spike: Spike,
    trigger_tx: TriggerTx,
}
impl Default for SpikeBundle {
    fn default() -> Self {
        Self {
            wait: MyLdtkWait::parent_render_layers(MainLayer::render_layers()),
            pos: Pos::new(-6000.0, -6000.0), // Will be overwritten
            spike: Spike,
            trigger_tx: TriggerTx::single(TriggerTxKind::Spike, Hbox::new().with_size(6, 6)),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct PassPlat;

#[derive(Bundle, LdtkIntCell)]
pub struct PassPlatBundle {
    wait: MyLdtkWait,
    pos: Pos,
    pass: PassPlat,
    static_tx: StaticTx,
}
impl Default for PassPlatBundle {
    fn default() -> Self {
        Self {
            wait: MyLdtkWait::parent_render_layers(MainLayer::render_layers()),
            pos: Pos::new(-6000.0, -6000.0), // Will be overwritten
            pass: PassPlat,
            static_tx: StaticTx::single(
                StaticTxKind::PassUp,
                Hbox::new().with_size(8, 4).with_offset(0.0, 2.0),
            ),
        }
    }
}
