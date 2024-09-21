use crate::prelude::*;

#[derive(Component)]
struct GuilloSpawnPoint;

#[derive(Bundle)]
struct GuilloSpawnPointBundle {
    name: Name,
    spawn_point: GuilloSpawnPoint,
    wait: MyLdtkWait,
    deps: MyLdtkDependents,
}

impl LdtkEntity for GuilloSpawnPointBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            name: Name::new("skelly_spawn_point"),
            spawn_point: GuilloSpawnPoint,
            wait: default(),
            deps: default(),
        }
    }
}

#[derive(Bundle)]
struct GuilloBundle {
    name: Name,
    anim: AnimMan<GuilloAnim>,
    trigger_tx: TriggerTx,
    pos: Pos,
    spatial: SpatialBundle,
}
impl GuilloBundle {
    fn from_pos(pos: Pos) -> Self {
        Self {
            name: Name::new("guillo"),
            anim: AnimMan::new(),
            trigger_tx: TriggerTx::single(TriggerTxKind::Guillo, Hbox::new().with_size(16, 32)),
            pos,
            spatial: pos.to_spatial(ZIX_GBOY - 3.2),
        }
    }
}

/// These are simple spawn points. Wait till they get a pos, spawn a single guy, then die
fn crush_spawns(
    mut commands: Commands,
    mut ents: Query<(Entity, &GuilloSpawnPoint, &mut MyLdtkDependents, &Pos)>,
    root: Res<LevelRoot>,
) {
    for (eid, _sp, mut deps, pos) in &mut ents {
        let dep = commands
            .spawn(GuilloBundle::from_pos(*pos))
            .set_parent(root.eid())
            .id();

        // Remove
        deps.push(dep);
        commands.entity(eid).remove::<GuilloSpawnPoint>();
    }
}

fn end_level(
    mut guillo: Query<(&TriggerTxCtrl, &mut AnimMan<GuilloAnim>)>,
    mut meta_state: ResMut<NextState<MetaState>>,
) {
    let Ok((trig, mut anim)) = guillo.get_single_mut() else {
        return;
    };
    if trig.coll_keys.len() > 0 {
        anim.set_state(GuilloAnim::Fall);
        meta_state.set(MenuState::WorldSelect.to_meta_state());
    }
}

pub(super) fn register_guillo(app: &mut App) {
    app.add_systems(PreUpdate, crush_spawns);
    app.register_ldtk_entity::<GuilloSpawnPointBundle>("Guillo");
    app.add_systems(Last, end_level.run_if(in_state(LevelStateKind::Playing)));
}
