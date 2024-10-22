use crate::prelude::*;

/// It's a little cumbersome to get this info
/// Nice to abstract away and update in PreUpdate for other systems that need it
#[derive(Resource, Debug, Clone, Reflect, Default)]
pub struct CurrentLevelHelpers {
    pub bounds: Option<Rect>,
}

#[derive(Event, Debug, Clone, Reflect)]
pub struct StartSwitchToLevel {
    iid: LevelIid,
}

/// Cases:
/// - GBoy doesn't exist, or exists and is in current level -> do nothing
/// - GBoy outside current level, but is in another level -> switch to that level
/// - GBoy outside current level, is not in another level -> invisible wall
fn change_current_level(
    mut gboy: Query<(Entity, &mut Pos, &mut Dyno), With<GBoy>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    helpers: Res<CurrentLevelHelpers>, // This should be correctly set _from the previous frame_
    level_selection: Res<LevelSelection>,
    mut commands: Commands,
) {
    // Do nothing cases
    let Ok((gboy_eid, mut gboy_pos, mut _dyno)) = gboy.get_single_mut() else {
        return;
    };
    let gboy_vec2 = gboy_pos.as_vec2();
    let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) else {
        return;
    };
    let Some(current_bounds) = helpers.bounds else {
        return;
    };
    if current_bounds.contains(gboy_vec2) {
        return;
    }
    // Gotta do something...
    let mut new_level_n_bounds: Option<&LevelIid> = None;
    for (level_iid, level_transform) in levels.iter() {
        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");
        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        if level_bounds.contains(gboy_vec2) {
            new_level_n_bounds = Some(level_iid);
            break;
        }
    }
    match new_level_n_bounds {
        Some(new_iid) => {
            if let LevelSelection::Iid(existing_iid) = level_selection.as_ref() {
                if new_iid == existing_iid {
                    return;
                }
            }
            commands.trigger(StartSwitchToLevel {
                iid: new_iid.clone(),
            });
        }
        None => {
            gboy_pos.x = gboy_pos.x.max(current_bounds.min.x);
            gboy_pos.x = gboy_pos.x.min(current_bounds.max.x);
            if gboy_pos.y < current_bounds.min.y {
                commands.entity(gboy_eid).insert(GBoyDying);
            }
        }
    }
}

fn handle_start_switch_to_level(
    trigger: Trigger<StartSwitchToLevel>,
    mut level_selection: ResMut<LevelSelection>,
) {
    let event = trigger.event();
    *level_selection = LevelSelection::Iid(event.iid.clone());
}

/// This ONLY updates the current level bounds
fn update_current_level_bounds(
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    mut helpers: ResMut<CurrentLevelHelpers>,
) {
    let mut new_bounds = None;
    let Ok(proj) = ldtk_projects.get_single() else {
        return;
    };
    if let Some(ldtk_project) = ldtk_project_assets.get(proj) {
        for (level_iid, level_transform) in levels.iter() {
            if LevelSelection::Iid(level_iid.clone()) != *level_selection {
                continue;
            }
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");
            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };
            new_bounds = Some(level_bounds);
        }
    }
    helpers.bounds = new_bounds;
}

#[derive(Resource)]
struct MinLevelLoadTime(f32);

fn start_load_level(
    mut commands: Commands,
    level_state: Res<State<LevelState>>,
    existing: Query<Entity, With<Handle<LdtkProject>>>,
    asset_server: Res<AssetServer>,
    mut level_selection: ResMut<LevelSelection>,
    mut min_level_load: ResMut<MinLevelLoadTime>,
) {
    let LevelState::Loading(loading_state) = level_state.get() else {
        panic!("bad load_level1");
    };
    for eid in &existing {
        commands.entity(eid).despawn_recursive();
    }
    commands.spawn((
        Name::new(format!("ldtk_world_{}", loading_state.world_path)),
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("ldtk/world.ldtk"),
            ..default()
        },
    ));
    *level_selection = LevelSelection::iid(loading_state.level_iid.to_string());
    min_level_load.0 = 0.0;
}

fn watch_stop_load_level(
    active_tombstone: Query<Entity, With<TombstoneActive>>,
    walls: Query<Entity, With<Wall>>,
    mut meta_state: ResMut<NextState<MetaState>>,
    mut min_load_time: ResMut<MinLevelLoadTime>,
    time: Res<Time>,
) {
    if min_load_time.0 < 0.3 {
        min_load_time.0 += time.delta_seconds();
        return;
    }
    if walls.iter().count() < 20 {
        return;
    }
    if active_tombstone.iter().count() == 1 {
        meta_state.set(LevelState::Spawning.to_meta_state());
    }
}

pub(super) fn register_my_ldtk_levels(app: &mut App) {
    reg_types!(app, CurrentLevelHelpers);

    app.insert_resource(CurrentLevelHelpers::default());
    app.add_systems(
        PreUpdate,
        (change_current_level, update_current_level_bounds).chain(),
    );

    app.add_systems(OnEnter(LevelStateKind::Loading), start_load_level);
    app.add_systems(
        Update,
        watch_stop_load_level.run_if(in_state(LevelStateKind::Loading)),
    );
    app.insert_resource(MinLevelLoadTime(0.0));

    app.observe(handle_start_switch_to_level);
}
