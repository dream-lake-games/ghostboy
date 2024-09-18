use crate::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub struct GBoyDying;
impl Component for GBoyDying {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, _, _| {
            world.commands().trigger(DoGBoyDeath);
        });
    }
}

#[derive(Event)]
struct DoGBoyDeath;

fn watch_for_death(
    gboy: Query<(Entity, &TriggerRxCtrl), (With<GBoy>, Without<GBoyDying>)>,
    mut commands: Commands,
    trigger_colls: Res<TriggerColls>,
) {
    let Ok((eid, tx_ctrl)) = gboy.get_single() else {
        return;
    };
    let trigs = trigger_colls.get_refs(&tx_ctrl.coll_keys);
    if trigs
        .iter()
        .any(|coll| coll.tx_kind == TriggerTxKind::Spike)
    {
        commands.entity(eid).insert(GBoyDying);
        return;
    }
}

fn handle_death(
    _trigger: Trigger<DoGBoyDeath>,
    mut meta_state: ResMut<NextState<MetaState>>,
    mut boys: Query<(Entity, &mut AnimMan<GBoyAnim>), With<GBoy>>,
    mut commands: Commands,
) {
    meta_state.set(LevelState::Dying.to_meta_state());
    for (eid, mut anim) in &mut boys {
        anim.set_state(GBoyAnim::Explode);
        anim.set_flip_x(thread_rng().gen());
        anim.set_flip_y(thread_rng().gen());
        commands.entity(eid).remove::<Dyno>();
        commands.entity(eid).remove::<TriggerTxCtrl>();
        commands.entity(eid).remove::<TriggerRxCtrl>();
    }
}

fn update_death(mut meta_state: ResMut<NextState<MetaState>>, boys: Query<Entity, With<GBoy>>) {
    if boys.is_empty() {
        meta_state.set(LevelState::Spawning.to_meta_state());
    }
}

pub(super) fn register_death(app: &mut App) {
    app.add_systems(
        Update,
        (watch_for_death)
            .after(PhysicsSet)
            .run_if(in_state(LevelState::Playing)),
    );
    app.add_systems(Update, (update_death).run_if(in_state(LevelState::Dying)));
    app.observe(handle_death);
}
