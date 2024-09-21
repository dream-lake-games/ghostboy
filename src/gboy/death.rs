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
    mut gboy: Query<(Entity, &mut AnimMan<GBoyAnim>, &Pos), With<GBoy>>,
    mut commands: Commands,
    mut cam_mode: ResMut<DynamicCameraMode>,
    mut fade: ResMut<Fade>,
    mut remaps: ResMut<ShadeRemaps>,
) {
    meta_state.set(LevelState::Dying.to_meta_state());
    let Ok((eid, mut anim, pos)) = gboy.get_single_mut() else {
        return;
    };
    anim.set_state(GBoyAnim::Explode);
    anim.set_flip_x(thread_rng().gen());
    anim.set_flip_y(thread_rng().gen());
    commands.entity(eid).remove::<Dyno>();
    commands.entity(eid).remove::<TriggerTxCtrl>();
    commands.entity(eid).remove::<TriggerRxCtrl>();
    *cam_mode = DynamicCameraMode::Hanging;
    fade.out(pos.clone());
    *remaps = default();
    let se = vec![
        SoundEffect::Death1,
        SoundEffect::Death2,
        SoundEffect::Death3,
    ]
    .pick();
    commands.spawn(se);
}

fn update_death(mut meta_state: ResMut<NextState<MetaState>>, boys: Query<Entity, With<GBoy>>) {
    if boys.is_empty() {
        meta_state.set(LevelState::Spawning.to_meta_state());
    }
}

fn update_ragdolls(
    mut ragdolls: Query<(Entity, &mut AnimMan<RagdollAnim>, &StaticRxTouches)>,
    mut commands: Commands,
) {
    for (eid, mut anim, touches) in &mut ragdolls {
        if (touches[Dir4::Down] == 0.0 && touches[Dir4::Up] > 0.2) || touches[Dir4::Down] > 3.0 {
            if anim.get_state() == RagdollAnim::Fall {
                anim.set_state(RagdollAnim::Land);
                commands.entity(eid).remove::<Dyno>();
            }
        }
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
    app.add_systems(Update, update_ragdolls);
}
