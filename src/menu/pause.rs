use crate::prelude::*;

#[derive(Component)]
struct PauseSprites;

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
struct ButtonKind(Action);

#[derive(Component, Clone)]
enum Action {
    BackToWorld,
    Unpause,
}

#[derive(Bundle)]
struct ButtonBundle {
    name: Name,
    anim: AnimMan<ButtonAnim>,
    spatial: SpatialBundle,
    kind: ButtonKind,
}
impl ButtonBundle {
    fn new(pos: Pos, action: Action) -> Self {
        Self {
            name: Name::new(format!("button")),
            anim: AnimMan::new().with_play_while_paused(true),
            spatial: pos.to_spatial(0.0),
            kind: ButtonKind(action),
        }
    }
}

fn on_enter(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn((
        Name::new("world_text_sprite"),
        SpriteBundle {
            texture: ass.load("menu/pause.png"),
            ..default()
        },
        MenuLayer::render_layers(),
        PauseSprites,
    ));
    commands.spawn((
        Name::new("world_text_sprite"),
        SpriteBundle {
            texture: ass.load("menu/controls.png"),
            ..default()
        },
        MenuLayer::render_layers(),
        PauseSprites,
    ));
    commands.spawn(SoundEffect::PauseIn);
    let mut _last_ent;
    // let mut new_ent = None;
    // macro_rules! add_level {
    //     ($pos:expr, $iid:expr) => {{
    //         let last_ent = &mut last_ent;
    //         let new_ent = &mut new_ent;
    //         *new_ent = Some(
    //             commands
    //                 .spawn(ButtonBundle::new($pos, $iid))
    //                 .insert(Prev(last_ent.clone().unwrap()))
    //                 .id(),
    //         );
    //         commands
    //             .entity(last_ent.clone().unwrap())
    //             .insert(Next(new_ent.clone().unwrap()));
    //         *last_ent = *new_ent;
    //     }};
    // }
    _last_ent = Some(
        commands
            .spawn(ButtonBundle::new(
                Pos::new(-28.0, -15.0),
                Action::BackToWorld,
            ))
            .insert(Selected)
            .id(),
    );
    // add_level!(Pos::new(0.0, 0.0), "52d3b660-4ce0-11ef-8383-f788b4218df1");
    // add_level!(Pos::new(40.0, 0.0), "6707e010-4ce0-11ef-8458-1d8de6fabb3d");
}

fn fixed_update(mut _commands: Commands) {
    // if thread_rng().gen_bool(0.5 / FRAMERATE as f64) {
    //     commands.trigger(Lightning);
    // }
}

fn update_input(
    dir4_input: Res<Dir4Input>,
    gbutton_input: Res<GButtonInput>,
    special_input: Res<SpecialButtonInput>,
    mut fade: ResMut<Fade>,
    mut meta_state: ResMut<NextState<MetaState>>,
    mut pause_state: ResMut<NextState<PauseState>>,
    current: Query<(Entity, &ButtonKind, Option<&Prev>, Option<&Next>), With<Selected>>,
    mut commands: Commands,
    action: Query<&Action>,
    gboy_pos: Query<&Pos, With<GBoy>>,
    mut camera_mode: ResMut<DynamicCameraMode>,
) {
    if action.is_empty() {
        let left = dir4_input.just_pressed(Dir4::Left);
        let right = dir4_input.just_pressed(Dir4::Right);
        let (eid, kind, oprev, onext) = current.single();
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

        if go_forward {
            commands.spawn(kind.0.clone());
            fade.out(gboy_pos.single().clone());
        }
        if go_back {
            commands.spawn(Action::Unpause);
            commands.spawn(SoundEffect::Select);
        }
    } else {
        let act = action.single();
        match act {
            Action::BackToWorld => {
                if fade.anim == FadeAnim::Black {
                    pause_state.set(PauseState::Unpaused);
                    meta_state.set(MenuState::WorldSelect.to_meta_state());
                    *camera_mode = DynamicCameraMode::Hanging;
                }
            }
            Action::Unpause => {
                pause_state.set(PauseState::Unpaused);
            }
        };
    }
}

fn on_exit(
    mut commands: Commands,
    ephemeral: Query<Entity, Or<(With<PauseSprites>, With<ButtonKind>, With<Action>)>>,
) {
    for ent in &ephemeral {
        commands.entity(ent).despawn_recursive();
    }
    commands.spawn(SoundEffect::PauseOut);
}

fn watch_for_pause(
    mut pause_state: ResMut<NextState<PauseState>>,
    special_input: Res<SpecialButtonInput>,
) {
    if special_input.just_pressed(SpecialButton::Select) {
        pause_state.set(PauseState::Paused);
    }
}

pub(super) fn register_pause(app: &mut App) {
    app.add_systems(OnEnter(PauseState::Paused), on_enter);
    app.add_systems(
        FixedUpdate,
        fixed_update.run_if(in_state(PauseState::Paused)),
    );
    app.add_systems(
        Update,
        update_input
            .after(InputSet)
            .run_if(in_state(PauseState::Paused)),
    );
    app.add_systems(
        Update,
        watch_for_pause.run_if(in_state(LevelStateKind::Playing)),
    );
    app.add_systems(OnExit(PauseState::Paused), on_exit);
}
