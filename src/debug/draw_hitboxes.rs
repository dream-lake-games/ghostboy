use crate::prelude::*;

trait HitboxColorable {
    fn to_color(&self) -> Color;
}

impl HitboxColorable for StaticRxKind {
    fn to_color(&self) -> Color {
        match self {
            Self::Default => tailwind::AMBER_200.into(),
        }
    }
}
impl HitboxColorable for StaticTxKind {
    fn to_color(&self) -> Color {
        match self {
            Self::Solid => tailwind::ORANGE_400.into(),
            Self::PassUp => tailwind::SLATE_300.into(),
        }
    }
}
impl HitboxColorable for TriggerRxKind {
    fn to_color(&self) -> Color {
        tailwind::GREEN_200.into()
    }
}
impl HitboxColorable for TriggerTxKind {
    fn to_color(&self) -> Color {
        tailwind::BLUE_200.into()
    }
}

fn draw_hitboxes(
    mut gz: Gizmos,
    pos_q: Query<&IPos>,
    static_rxs: Query<&StaticRxComp>,
    static_txs: Query<&StaticTxComp>,
    trigger_rxs: Query<&TriggerRxComp>,
    trigger_txs: Query<&TriggerTxComp>,
) {
    macro_rules! handle_comp {
        ($comp:expr) => {
            for comp in &$comp {
                let color = comp.kind.to_color();
                let Ok(ipos) = pos_q.get(comp.ctrl) else {
                    continue;
                };
                let pos = ipos.cur.as_vec2() + comp.hbox.get_offset();
                gz.rect_2d(pos, Rot2::default(), comp.hbox.get_size().as_vec2(), color);
            }
        };
    }
    handle_comp!(static_rxs);
    handle_comp!(static_txs);
    handle_comp!(trigger_rxs);
    handle_comp!(trigger_txs);
}

pub(super) fn register_draw_hitboxes(app: &mut App) {
    app.add_systems(
        PostUpdate,
        draw_hitboxes
            .after(PhysicsSet)
            .run_if(input_toggle_active(false, KeyCode::KeyH)),
    );
}
