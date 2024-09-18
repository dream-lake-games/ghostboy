use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
struct Tombstone {
    iid: String,
}
#[derive(Clone, Debug)]
pub struct TombstoneActive {
    level_selection: LevelSelection,
}
impl Component for TombstoneActive {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            // Then update the animation
            let mut anim = world.get_mut::<AnimMan<TombstoneAnim>>(eid).unwrap();
            anim.set_state(TombstoneAnim::Reach);
        });
        hooks.on_remove(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<TombstoneAnim>>(eid).unwrap();
            anim.set_state(TombstoneAnim::Inactive);
        });
    }
}
#[derive(Component, Clone, Debug, Reflect)]
struct TombstoneReached;

#[derive(Component, Default)]
struct TombstoneHere {
    iid: String,
    is_initial: bool,
}
#[derive(Bundle)]
struct TombstoneHereBundle {
    marker: TombstoneHere,
    wait: MyLdtkWait,
}
impl LdtkEntity for TombstoneHereBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let fi = entity_instance
            .get_field_instance("is_initial")
            .expect("no is_initial on tomb");
        let FieldValue::Bool(is_initial) = fi.value else {
            panic!("woop bad tombstone!");
        };
        Self {
            marker: TombstoneHere {
                iid: entity_instance.iid.clone(),
                is_initial,
            },
            wait: default(),
        }
    }
}

fn materialize_tombstones(
    query: Query<(Entity, &TombstoneHere, &Pos)>,
    mut commands: Commands,
    root: Res<LevelRoot>,
    level_selection: Res<LevelSelection>,
    existing: Query<&Tombstone>,
) {
    for (eid, here, pos) in &query {
        if !existing.iter().any(|tomb| tomb.iid == here.iid) {
            let mut comms = commands.spawn(TombstoneBundle::new(here.iid.clone(), pos.clone()));
            comms.set_parent(root.eid());
            if here.is_initial {
                comms.insert((
                    TombstoneActive {
                        level_selection: level_selection.clone(),
                    },
                    TombstoneReached,
                ));
            }
        }
        commands.entity(eid).despawn_recursive();
    }
}

#[derive(Bundle)]
struct TombstoneBundle {
    name: Name,
    marker: Tombstone,
    anim: AnimMan<TombstoneAnim>,
    pos: Pos,
    spatial: SpatialBundle,
    trigger_rx: TriggerRx,
}
impl TombstoneBundle {
    pub fn new(iid: String, pos: Pos) -> Self {
        let hacked_pos = Pos::new(pos.x, pos.y + 4.0);
        Self {
            name: Name::new("tombstone"),
            marker: Tombstone { iid },
            anim: AnimMan::new(),
            pos: hacked_pos,
            spatial: hacked_pos.to_spatial(ZIX_TOMBSTONE),
            trigger_rx: TriggerRx::single(
                TriggerRxKind::Tombstone,
                Hbox::new().with_size(16, 16).with_offset(0.0, -4.0),
            ),
        }
    }
}

fn reach_and_activate_tombstones(
    trigger_colls: Res<TriggerColls>,
    waiting_q: Query<(Entity, &TriggerRxCtrl), (With<Tombstone>, Without<TombstoneActive>)>,
    mut commands: Commands,
    current_active: Query<Entity, With<TombstoneActive>>,
    level_selection: Res<LevelSelection>,
) {
    let mut new_active = None;
    for (eid, ctrl) in &waiting_q {
        let colls = trigger_colls.get_refs(&ctrl.coll_keys);
        if colls.iter().any(|coll| coll.tx_kind == TriggerTxKind::GBoy) {
            commands.entity(eid).insert(TombstoneReached);
            new_active = Some(eid);
        }
    }
    if let Some(new_eid) = new_active {
        for old_active in &current_active {
            commands.entity(old_active).remove::<TombstoneActive>();
        }
        commands.entity(new_eid).insert(TombstoneActive {
            level_selection: level_selection.clone(),
        });
    }
}

fn tombstone_spawn(
    active: Query<(&Pos, &TombstoneActive)>,
    mut commands: Commands,
    root: Res<LevelRoot>,
    mut level_selection: ResMut<LevelSelection>,
    lingering: Query<Entity, With<GBoy>>,
) {
    let Ok((pos, active)) = active.get_single() else {
        return;
    };
    *level_selection = active.level_selection.clone();
    for eid in &lingering {
        commands.entity(eid).despawn_recursive();
    }
    commands
        .spawn(super::GBoyBundle::new(pos.clone()))
        .set_parent(root.eid());
}

fn finish_spawning(gboy: Query<&AnimMan<GBoyAnim>>, mut meta_state: ResMut<NextState<MetaState>>) {
    let Ok(anim) = gboy.get_single() else {
        return;
    };
    // TODO: Make a spawning animation
    if anim.get_state() == GBoyAnim::Stand {
        meta_state.set(LevelState::Playing.to_meta_state());
    }
}

pub(super) fn register_spawn(app: &mut App) {
    app.register_ldtk_entity_for_layer::<TombstoneHereBundle>("Entities", "Tombstone");
    app.add_systems(OnEnter(LevelStateKind::Spawning), tombstone_spawn);
    app.add_systems(
        Update,
        finish_spawning.run_if(in_state(LevelState::Spawning)),
    );

    app.add_systems(Update, materialize_tombstones);
    app.add_systems(Update, reach_and_activate_tombstones.after(PhysicsSet));
}
