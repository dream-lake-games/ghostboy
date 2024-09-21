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

#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct EphPlat;

#[derive(Bundle, Debug, Clone, Reflect)]
pub struct EphPlatBundle {
    name: Name,
    pos: Pos,
    eph: EphPlat,
    static_tx: StaticTx,
    trigger_tx: TriggerTx,
    anim: AnimMan<EphPlatAnim>,
}
impl EphPlat {
    fn hbox() -> Hbox {
        Hbox::new().with_size(8, 8)
    }
}
impl Queryable for EphPlatBundle {}
impl MyLdtkReplacable for EphPlatBundle {
    fn from_pos(pos: Pos) -> Self {
        Self {
            name: Name::new("eph_plat"),
            pos,
            eph: EphPlat,
            static_tx: StaticTx::single(StaticTxKind::Solid, Hbox::new().with_size(8, 8)),
            trigger_tx: TriggerTx::single(TriggerTxKind::Dummy, EphPlat::hbox()),
            anim: AnimMan::new(),
        }
    }
}

fn update_eph_plats(
    mut plats: Query<(&mut AnimMan<EphPlatAnim>, &StaticTxCtrl, &TriggerTxCtrl), With<EphPlat>>,
    mut comps: Query<&mut StaticTxComp>,
    colls: Res<StaticColls>,
) {
    for (mut anim, stx_ctrl, ttx_colls) in &mut plats {
        if anim.get_state() == EphPlatAnim::Refill {
            for cid in &stx_ctrl.comps {
                let mut comp = comps.get_mut(*cid).unwrap();
                comp.hbox = Hbox::new().translated(-10000.0, -10000.0);
            }
        } else {
            let current_hbox = comps.get(stx_ctrl.comps[0]).unwrap().hbox.clone();
            if current_hbox.get_size().x == 0 {
                if ttx_colls.coll_keys.len() == 0 {
                    let mut comp = comps.get_mut(stx_ctrl.comps[0]).unwrap();
                    comp.hbox = EphPlat::hbox();
                }
            } else {
                if colls
                    .get_refs(&stx_ctrl.coll_keys)
                    .iter()
                    .any(|coll| coll.rx_perp.y < 0.0)
                {
                    anim.set_state(EphPlatAnim::Fade);
                }
            }
        }
    }
}

pub(super) fn register_wallish(app: &mut App) {
    app.register_ldtk_int_cell_for_layer::<WallBundle>("Ground", 1)
        .register_ldtk_int_cell_for_layer::<WallBundle>("Platform", 1);
    for val in [1, 2, 3, 4, 5, 6] {
        app.register_ldtk_int_cell_for_layer::<SpikeBundle>("Spikes", val);
    }
    app.register_ldtk_int_cell_for_layer::<PassPlatBundle>("PassPlat", 1);
    app.register_ldtk_int_cell_for_layer::<MyLdtkReplace<EphPlatBundle>>("Eph", 1);

    app.add_systems(Update, update_eph_plats.after(PhysicsSet));

    register_replaceable::<EphPlatBundle>(app);
}
