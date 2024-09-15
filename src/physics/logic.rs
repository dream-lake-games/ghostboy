use crate::prelude::*;

/// A helpful function to make sure physics things exist as we expect them to
fn invariants(
    dyno_without_pos: Query<Entity, (With<Dyno>, Without<Pos>)>,
    static_rx_n_tx: Query<Entity, (With<StaticRxCtrl>, With<StaticTxCtrl>)>,
) {
    debug_assert!(dyno_without_pos.is_empty());
    debug_assert!(static_rx_n_tx.is_empty());
}

/// Moves dynos that have no statics or triggers
fn move_uninteresting_dynos(
    bullet_time: Res<BulletTime>,
    mut ents: Query<
        (&Dyno, &mut Pos),
        (
            Without<StaticRxCtrl>,
            Without<StaticTxCtrl>,
            Without<TriggerRxCtrl>,
            Without<TriggerTxCtrl>,
        ),
    >,
) {
    for (dyno, mut pos) in &mut ents {
        *pos += dyno.vel * bullet_time.delta_seconds();
    }
}

/// TODO! Moves StaticTxs
fn move_static_txs(
    mut ents: Query<(&Dyno, &mut Pos), (Without<StaticRxCtrl>, With<StaticTxCtrl>)>,
) {
    for (_dyno, mut _pos) in &mut ents {
        todo!("Do we want this? How should it work?");
    }
}

/// Resolves collisions for a single entity.
/// If it has statics, it resolves static collisions and may update pos and vel
/// If it has triggers, it will trigger as needed (duh)
fn resolve_collisions(
    my_eid: Entity,
    my_pos: &mut Pos,
    my_vel: &mut Vec2,
    my_srx_comps: &[&StaticRxComp],
    my_trx_comps: &[&TriggerRxComp],
    my_ttx_comps: &[&TriggerTxComp],
    pos_q: &Query<
        &mut Pos,
        Or<(
            With<StaticRxCtrl>,
            With<StaticTxCtrl>,
            With<TriggerRxCtrl>,
            With<TriggerTxCtrl>,
        )>,
    >,
    stx_comps: &Query<&StaticTxComp>,
    trx_comps: &Query<&TriggerRxComp>,
    ttx_comps: &Query<&TriggerTxComp>,
) {
    macro_rules! translate_other {
        ($comp:expr) => {{
            let tmp_pos = pos_q
                .get($comp.ctrl)
                .expect("Bad pos in translate_other")
                .clone();
            $comp.hbox.translated(tmp_pos.x, tmp_pos.y)
        }};
    }

    for my_srx_comp in my_srx_comps {
        let mut my_thbox = my_srx_comp.hbox.translated(my_pos.x, my_pos.y);
        for other_stx_comp in stx_comps {
            if other_stx_comp.ctrl == my_eid {
                // Don't collide with ourselves, stupid
                continue;
            }
            let other_thbox = translate_other!(other_stx_comp);
            if let Some(push) = my_thbox.get_push_out(&other_thbox) {
                *my_pos += push;
                // NOTE: HAVE TO UPDATE MY_THBOX HERE SINCE POS CHANGED
                my_thbox = my_thbox.translated(push.x, push.y);

                let old_perp = my_vel.dot(push.normalize_or_zero()) * push.normalize_or_zero();
                let old_par = *my_vel - old_perp;

                match (my_srx_comp.kind, other_stx_comp.kind) {
                    (StaticRxKind::Default, StaticTxKind::Solid) => {
                        *my_vel = old_par;
                        if old_perp.dot(push) > 0.0 {
                            *my_vel += old_perp;
                        }
                    }
                }
            }
        }
    }
}

/// Moves the interesting stuff and handles collisions
fn move_interesting_dynos(
    bullet_time: Res<BulletTime>,
    mut pos_q: Query<
        &mut Pos,
        Or<(
            With<StaticRxCtrl>,
            With<StaticTxCtrl>,
            With<TriggerRxCtrl>,
            With<TriggerTxCtrl>,
        )>,
    >,
    mut dyno_q: Query<
        &mut Dyno,
        Or<(
            With<StaticRxCtrl>,
            With<StaticTxCtrl>,
            With<TriggerRxCtrl>,
            With<TriggerTxCtrl>,
        )>,
    >,
    srx_comps: Query<&StaticRxComp>,
    stx_comps: Query<&StaticTxComp>,
    trx_comps: Query<&TriggerRxComp>,
    ttx_comps: Query<&TriggerTxComp>,
    // Objects that have a static receiver. They may also have triggers.
    ents: Query<
        (
            Entity,
            Option<&StaticRxCtrl>,
            Option<&TriggerRxCtrl>,
            Option<&TriggerTxCtrl>,
        ),
        (
            With<Dyno>,
            Without<StaticTxCtrl>,
            Or<(With<StaticRxCtrl>, With<TriggerRxCtrl>, With<TriggerTxCtrl>)>,
        ),
    >,
) {
    // First move static rxs
    for (eid, srx_ctrl, trx_ctrl, ttx_ctrl) in &ents {
        // Get the data
        let mut scratch_pos = pos_q.get(eid).expect("No pos on interesting ent").clone();
        let mut scratch_vel = dyno_q
            .get(eid)
            .expect("No dyno on static interesting ent")
            .vel
            .clone();
        macro_rules! get_comps {
            ($ctrl:expr, $comp_query:expr) => {{
                $ctrl
                    .map(|ctrl| {
                        ctrl.comps
                            .iter()
                            .map(|comp_eid| $comp_query.get(*comp_eid))
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or(vec![])
            }};
        }
        let my_srx_comps = get_comps!(srx_ctrl, srx_comps);
        let my_trx_comps = get_comps!(trx_ctrl, trx_comps);
        let my_ttx_comps = get_comps!(ttx_ctrl, ttx_comps);
        // Inch
        const DELTA_PER_INCH: f32 = 1.0;
        // Resolve collisions once always so stationary objects are still pushed out of each other
        resolve_collisions(
            eid,
            &mut scratch_pos,
            &mut scratch_vel,
            &my_srx_comps,
            &my_trx_comps,
            &my_ttx_comps,
            &pos_q,
            &stx_comps,
            &trx_comps,
            &ttx_comps,
        );
        // Inch horizontally
        let mut amt_moved_hor: f32 = 0.0;
        let max_inch_hor = scratch_vel.x.abs() * bullet_time.delta_seconds();
        while amt_moved_hor < max_inch_hor.min(scratch_vel.x.abs()) {
            let dont_overshoot = (max_inch_hor.min(scratch_vel.x.abs()) - amt_moved_hor).max(0.0);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_hor += moving_this_step;
            scratch_pos.x += scratch_vel.x.signum() * moving_this_step;
            resolve_collisions(
                eid,
                &mut scratch_pos,
                &mut scratch_vel,
                &my_srx_comps,
                &my_trx_comps,
                &my_ttx_comps,
                &pos_q,
                &stx_comps,
                &trx_comps,
                &ttx_comps,
            );
        }
        // Then inch vertically
        let mut amt_moved_ver: f32 = 0.0;
        let max_inch_ver = scratch_vel.y.abs() * bullet_time.delta_seconds();
        while amt_moved_ver < max_inch_ver.min(scratch_vel.y.abs()) {
            let dont_overshoot = (max_inch_ver.min(scratch_vel.y.abs()) - amt_moved_ver).max(0.0);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_ver += moving_this_step;
            scratch_pos.y += scratch_vel.y.signum() * moving_this_step;
            resolve_collisions(
                eid,
                &mut scratch_pos,
                &mut scratch_vel,
                &my_srx_comps,
                &my_trx_comps,
                &my_ttx_comps,
                &pos_q,
                &stx_comps,
                &trx_comps,
                &ttx_comps,
            );
        }
        // NOTE: Why do this (inch horizontally then vertically)? Stops bugs going up and down against wall.
        // ^read: celeste does this
        // Set the data
        let mut set_pos = pos_q.get_mut(eid).expect("No pos on interesting ent");
        let mut set_dyno = dyno_q.get_mut(eid).expect("No dyno on interesting ent");
        *set_pos = scratch_pos;
        set_dyno.vel = scratch_vel;
    }
}

pub(super) fn register_logic(app: &mut App) {
    app.add_systems(
        Update,
        (
            invariants,
            move_uninteresting_dynos,
            move_static_txs,
            move_interesting_dynos,
        )
            .chain()
            .in_set(PhysicsSet)
            .in_set(super::CollisionSet)
            .before(super::PosSet),
    );
}
