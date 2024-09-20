use crate::prelude::*;

#[derive(Component)]
struct SkellySpawnPoint {
    card_dir: CardDir,
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
            .get_field_instance("card_dir")
            .expect("Skelly has no card_dir");
        let card_dir = CardDir::from_field_instance(fi);
        Self {
            name: Name::new("skelly_spawn_point"),
            spawn_point: SkellySpawnPoint { card_dir },
            wait: default(),
            deps: default(),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Skelly {
    can_fire: bool,
    card_dir: CardDir,
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
        let translated = pos.translated(Vec2::new(0.0, 4.0));
        Self {
            name: Name::new("skelly"),
            skelly: Skelly {
                can_fire: true,
                card_dir: sp.card_dir,
            },
            anim: AnimMan::new(),
            facing: Facing::from_card_dir(sp.card_dir),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                Hbox::new().with_size(8, 14).with_offset(1.0, -1.0),
            ),
            pos: translated,
            dyno: default(),
            spatial: translated.to_spatial(ZIX_SKELLY),
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
        &mut AnimMan<SkellyAnim>,
        &AnimBodyProgress<SkellyAnim>,
    )>,
    root: Res<LevelRoot>,
) {
    for (pos, mut skelly, mut anim, anim_progress) in &mut skelly {
        macro_rules! check_pair {
            ($anim:expr, $anim_progress:expr, $upper:ident, $lower:ident) => {{
                paste::paste! {
                    $anim.get_state() == SkellyAnim::[<Fire $upper>] && $anim_progress.get_body_ix(AnimBody_SkellyAnim::[<fire_ $lower>]) == Some(9)
                }
            }};
        }
        if check_pair!(anim, anim_progress, NE, ne)
            || check_pair!(anim, anim_progress, N, n)
            || check_pair!(anim, anim_progress, E, e)
            || check_pair!(anim, anim_progress, SE, se)
        {
            if skelly.can_fire {
                skelly.can_fire = false;
                commands
                    .spawn(ArrowBundle::new(pos.clone(), skelly.card_dir))
                    .set_parent(root.eid());
            }
        } else {
            skelly.can_fire = true;
        }
        if anim.get_state() == SkellyAnim::Restart {
            let new_state = match skelly.card_dir {
                CardDir::N => SkellyAnim::FireN,
                CardDir::E | CardDir::W => SkellyAnim::FireE,
                CardDir::NE | CardDir::NW => SkellyAnim::FireNE,
                CardDir::SE | CardDir::SW => SkellyAnim::FireSE,
                _ => panic!("no south"),
            };
            anim.set_state(new_state);
        }
    }
}

pub(super) fn register_skelly(app: &mut App) {
    app.register_ldtk_entity_for_layer::<SkellySpawnPointBundle>("Entities", "SkellySpawn");

    app.add_systems(PreUpdate, crush_spawns);
    app.add_systems(
        Update,
        (fire_arrows)
            .in_set(EnemySet)
            .run_if(in_state(MetaStateKind::Level)),
    );
}
