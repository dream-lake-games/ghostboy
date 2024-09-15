use crate::prelude::*;

#[derive(Component)]
struct SkellySpawnPoint {
    facing: Facing,
}

#[derive(Bundle)]
struct SkellySpawnPointBundle {
    name: Name,
    spawn_point: SkellySpawnPoint,
    wait: MyLdtkWait,
    deps: MyLdtkDependents,
}
impl LdtkEntity for SkellySpawnPointBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let fi = entity_instance
            .get_field_instance("dir")
            .expect("Skelly has no dir");
        let dir = Dir4::from_field_instance(fi);
        Self {
            name: Name::new("skelly_spawn_point"),
            spawn_point: SkellySpawnPoint {
                facing: Facing::from_dir4(dir),
            },
            wait: default(),
            deps: default(),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Skelly;

#[derive(Bundle)]
struct SkellyBundle {
    name: Name,
    skelly: Skelly,
    anim: AnimMan<SkellyAnim>,
    facing: Facing,
    static_rx: StaticRx,
    pos: Pos,
    dyno: Dyno,
    spatial: SpatialBundle,
}
impl SkellyBundle {
    fn from_spawn_point(sp: &SkellySpawnPoint, pos: Pos) -> Self {
        Self {
            name: Name::new("skelly"),
            skelly: Skelly,
            anim: AnimMan::new(),
            facing: sp.facing,
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                Hbox::new().with_size(8, 14).with_offset(1.0, -1.0),
            ),
            pos,
            dyno: default(),
            spatial: pos.to_spatial(ZIX_SKELLY),
        }
    }
}

/// These are simple spawn points. Wait till they get a pos, spawn a single guy, then die
fn crush_spawns(
    mut commands: Commands,
    mut ents: Query<(Entity, &SkellySpawnPoint, &mut MyLdtkDependents, &Pos)>,
    root: Res<LevelRoot>,
) {
    for (eid, sp, mut deps, pos) in &mut ents {
        let dep = commands
            .spawn(SkellyBundle::from_spawn_point(sp, *pos))
            .set_parent(root.eid())
            .id();

        // Remove
        deps.push(dep);
        commands.entity(eid).remove::<SkellySpawnPoint>();
    }
}

pub(super) fn register_skelly(app: &mut App) {
    app.register_ldtk_entity_for_layer::<SkellySpawnPointBundle>("Entities", "SkellySpawn");

    app.add_systems(PreUpdate, crush_spawns);
}
