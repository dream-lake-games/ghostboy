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
pub struct Skelly {
    can_fire: bool,
}

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
            skelly: Skelly { can_fire: true },
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

fn fire_arrows(
    mut commands: Commands,
    mut skelly: Query<(
        &Pos,
        &mut Skelly,
        &AnimMan<SkellyAnim>,
        &AnimBodyProgress<SkellyAnim>,
    )>,
) {
    for (pos, mut skelly, anim_man, anim_progress) in &mut skelly {
        if anim_man.get_state() == SkellyAnim::Fire
            && anim_progress.get_body_ix(AnimBody_SkellyAnim::fire) == Some(11)
        {
            if skelly.can_fire {
                skelly.can_fire = false;
                commands.spawn(ArrowBundle::new(pos.clone(), CardDir::NE));
            }
        } else {
            skelly.can_fire = true;
        }
    }
}

pub(super) fn register_skelly(app: &mut App) {
    app.register_ldtk_entity_for_layer::<SkellySpawnPointBundle>("Entities", "SkellySpawn");

    app.add_systems(PreUpdate, crush_spawns);
    app.add_systems(Update, (fire_arrows).in_set(EnemySet));
}
