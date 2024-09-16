use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
struct Tombstone;
#[derive(Clone, Debug, Reflect)]
struct TombstoneActive;
impl Component for TombstoneActive {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<TombstoneAnim>>(eid).unwrap();
            if anim.get_state() != TombstoneAnim::Reach {
                anim.set_state(TombstoneAnim::Active);
            }
        });
        hooks.on_remove(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<TombstoneAnim>>(eid).unwrap();
            anim.set_state(TombstoneAnim::Inactive);
        });
    }
}
#[derive(Clone, Debug, Reflect)]
struct TombstoneReached;
impl Component for TombstoneReached {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<TombstoneAnim>>(eid).unwrap();
            anim.set_state(TombstoneAnim::Reach);
        });
    }
}

#[derive(Component, Default)]
struct TombstoneHere {
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
            marker: TombstoneHere { is_initial },
            wait: default(),
        }
    }
}

fn materialize_tombstones(
    query: Query<(Entity, &TombstoneHere, &Pos)>,
    mut commands: Commands,
    root: Res<LevelRoot>,
) {
    for (eid, here, pos) in &query {
        let mut comms = commands.spawn(TombstoneBundle::new(pos.clone()));
        comms.set_parent(root.eid());
        if here.is_initial {
            comms.insert((TombstoneActive, TombstoneReached));
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
    pub fn new(pos: Pos) -> Self {
        let hacked_pos = Pos::new(pos.x, pos.y + 4.0);
        Self {
            name: Name::new("tombstone"),
            marker: Tombstone,
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

fn tombstone_spawn(
    active: Query<&Pos, With<TombstoneActive>>,
    mut commands: Commands,
    root: Res<LevelRoot>,
) {
    let Ok(pos) = active.get_single() else {
        return;
    };
    commands
        .spawn(super::GBoyBundle::new(pos.clone()))
        .set_parent(root.eid());
}

pub(super) fn register_spawn(app: &mut App) {
    app.register_ldtk_entity_for_layer::<TombstoneHereBundle>("Entities", "Tombstone");
    app.add_systems(PreUpdate, tombstone_spawn.run_if(no_gboy_exists));
    app.add_systems(Update, materialize_tombstones);
}
