use crate::prelude::*;

fn static_coll_sounds(colls: Res<StaticColls>, mut commands: Commands, gboy: Query<&GBoy>) {
    for coll in colls.all() {
        if !gboy.contains(coll.rx_ctrl) {
            continue;
        }
        match (coll.rx_kind, coll.tx_kind) {
            (StaticRxKind::Default, StaticTxKind::PassUp)
            | (StaticRxKind::Default, StaticTxKind::Solid) => {
                const MIN: f32 = 70.0;
                const MAX: f32 = 300.0;
                let vol_mult = (coll.rx_perp.length().clamp(MIN, MAX) - MIN) / (MAX - MIN);
                if vol_mult > 0.01 {
                    let kinds = match coll.tx_kind {
                        StaticTxKind::PassUp => vec![SoundEffect::Normal1],
                        _ => vec![SoundEffect::Normal2, SoundEffect::Normal3],
                    };
                    commands.spawn((kinds.pick(), SoundMult(vol_mult), OneSound::Ignore));
                }
            }
        }
    }
}

pub(super) fn juice_sounds(app: &mut App) {
    app.add_systems(PostUpdate, static_coll_sounds.after(PhysicsSet));
}
