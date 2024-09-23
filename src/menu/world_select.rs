use crate::prelude::*;

#[derive(Component)]
struct WorldSprites;

#[derive(Component)]
struct Iid {
    iid: String,
}

struct Selected;
impl Component for Selected {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<ButtonAnim>>(eid).unwrap();
            anim.set_state(ButtonAnim::Select);
        });
        hooks.on_remove(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<ButtonAnim>>(eid).unwrap();
            anim.set_state(ButtonAnim::Empty);
        });
    }
}

#[derive(Component)]
struct Next(Entity);

#[derive(Component)]
struct Prev(Entity);

#[derive(Component)]
enum Action {
    GoBack,
    GoForward(String),
}

#[derive(Bundle)]
struct ButtonBundle {
    name: Name,
    anim: AnimMan<ButtonAnim>,
    spatial: SpatialBundle,
    level_iid: Iid,
}
impl ButtonBundle {
    fn new(pos: Pos, iid: &str) -> Self {
        Self {
            name: Name::new(format!("button_{iid}")),
            anim: AnimMan::new(),
            spatial: pos.to_spatial(0.0),
            level_iid: Iid {
                iid: iid.to_string(),
            },
        }
    }
}

fn on_enter(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut fade: ResMut<Fade>,
    proj: Query<Entity, With<Handle<LdtkProject>>>,
    level_root: Res<LevelRoot>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
) {
    for ent in &proj {
        commands.entity(ent).despawn_recursive();
    }
    commands.entity(level_root.eid()).despawn_descendants();
    commands.spawn((
        Name::new("world_text_sprite"),
        SpriteBundle {
            texture: ass.load("menu/world.png"),
            ..default()
        },
        MenuLayer::render_layers(),
        WorldSprites,
    ));
    commands.spawn((
        Name::new("world_text_sprite"),
        SpriteBundle {
            texture: ass.load("menu/controls.png"),
            ..default()
        },
        MenuLayer::render_layers(),
        WorldSprites,
    ));
    let mut last_ent;
    let mut new_ent = None;
    macro_rules! add_level {
        ($pos:expr, $iid:expr) => {{
            let last_ent = &mut last_ent;
            let new_ent = &mut new_ent;
            *new_ent = Some(
                commands
                    .spawn(ButtonBundle::new($pos, $iid))
                    .insert(Prev(last_ent.clone().unwrap()))
                    .id(),
            );
            commands
                .entity(last_ent.clone().unwrap())
                .insert(Next(new_ent.clone().unwrap()));
            *last_ent = *new_ent;
        }};
    }
    last_ent = Some(
        commands
            .spawn(ButtonBundle::new(
                Pos::new(-40.0, 0.0),
                "6707e010-4ce0-11ef-8458-1d8de6fabb3d",
            ))
            .insert(Selected)
            .id(),
    );
    add_level!(Pos::new(0.0, 0.0), "f3454410-73f0-11ef-8383-35f2180fb972");
    add_level!(Pos::new(40.0, 0.0), "bf908b30-73f0-11ef-8383-cb437c1cb6fb");
    if fade.anim == FadeAnim::Black {
        fade.in_(cam_pos.single().clone());
    }
}

fn fixed_update(mut commands: Commands) {
    if thread_rng().gen_bool(0.5 / FRAMERATE as f64) {
        commands.trigger(Lightning);
    }
}

fn update_input(
    dir4_input: Res<Dir4Input>,
    gbutton_input: Res<GButtonInput>,
    special_input: Res<SpecialButtonInput>,
    mut fade: ResMut<Fade>,
    mut meta_state: ResMut<NextState<MetaState>>,
    current: Query<(Entity, &Iid, Option<&Prev>, Option<&Next>), With<Selected>>,
    mut commands: Commands,
    action: Query<&Action>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
) {
    if action.is_empty() {
        let left = dir4_input.just_pressed(Dir4::Left);
        let right = dir4_input.just_pressed(Dir4::Right);
        let (eid, iid, oprev, onext) = current.single();
        if left && oprev.is_some() {
            commands.entity(eid).remove::<Selected>();
            commands.entity(oprev.unwrap().0).insert(Selected);
        }
        if right && onext.is_some() {
            commands.entity(eid).remove::<Selected>();
            commands.entity(onext.unwrap().0).insert(Selected);
        }

        let go_back = gbutton_input.just_pressed(GButton::B)
            || special_input.just_pressed(SpecialButton::Select);
        let go_forward = gbutton_input.just_pressed(GButton::A)
            || special_input.just_pressed(SpecialButton::Start);

        if go_back {
            fade.out(cam_pos.single().clone());
            commands.spawn(Action::GoBack);
            commands.spawn(SoundEffect::Select);
        }
        if go_forward {
            fade.out(cam_pos.single().clone());
            commands.spawn(Action::GoForward(iid.iid.clone()));
            commands.spawn(SoundEffect::Select);
        }
    } else {
        if fade.anim == FadeAnim::Black {
            let act = action.single();
            let next_state = match act {
                Action::GoBack => MenuState::Title.to_meta_state(),
                Action::GoForward(iid) => LevelState::Loading(LevelLoadingState {
                    world_path: "ldtk/world.ldtk".into(),
                    level_iid: LevelIid::new(iid),
                })
                .to_meta_state(),
            };
            meta_state.set(next_state);
        }
    }
}

fn on_exit(
    mut commands: Commands,
    ephemeral: Query<Entity, Or<(With<WorldSprites>, With<Iid>, With<Action>)>>,
) {
    for ent in &ephemeral {
        commands.entity(ent).despawn_recursive();
    }
}

pub(super) fn register_world_select(app: &mut App) {
    app.add_systems(OnEnter(MenuState::WorldSelect), on_enter);
    app.add_systems(
        FixedUpdate,
        fixed_update.run_if(in_state(MenuState::WorldSelect)),
    );
    app.add_systems(
        Update,
        update_input
            .after(InputSet)
            .run_if(in_state(MenuState::WorldSelect)),
    );
    app.add_systems(OnExit(MenuState::WorldSelect), on_exit);
}
