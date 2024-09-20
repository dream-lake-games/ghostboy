use crate::prelude::*;

#[derive(Component)]
struct RedeathSprite;

#[derive(Component)]
struct WentForward;

fn on_enter(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut fade: ResMut<Fade>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
    mut music: ResMut<MusicManager>,
) {
    commands.spawn((
        Name::new("redeath_title_sprite"),
        SpriteBundle {
            texture: ass.load("menu/redeath.png"),
            ..default()
        },
        MenuLayer::render_layers(),
        RedeathSprite,
    ));
    fade.in_(cam_pos.get_single().unwrap_or(&Pos::new(0.0, 0.0)).clone());
    music.fade_to_song(MusicKind::Draft);
}

fn fixed_update(mut commands: Commands) {
    if thread_rng().gen_bool(0.3 / FRAMERATE as f64) {
        commands.trigger(Lightning);
    }
}

fn update(
    gbutton_input: Res<GButtonInput>,
    special_input: Res<SpecialButtonInput>,
    mut fade: ResMut<Fade>,
    mut meta_state: ResMut<NextState<MetaState>>,
    went_forward: Query<&WentForward>,
    mut commands: Commands,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
) {
    let go_forward =
        special_input.pressed(SpecialButton::Start) || gbutton_input.pressed(GButton::A);
    if go_forward && fade.anim == FadeAnim::Clear {
        fade.out(cam_pos.single().clone());
        commands.spawn(WentForward);
    }
    if !went_forward.is_empty() {
        if fade.anim == FadeAnim::Black {
            meta_state.set(MenuState::WorldSelect.to_meta_state());
        }
    }
}

fn on_exit(
    mut commands: Commands,
    old: Query<Entity, Or<(With<RedeathSprite>, With<WentForward>)>>,
) {
    for ent in &old {
        commands.entity(ent).despawn_recursive();
    }
}

pub(super) fn register_title(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Title), on_enter);
    app.add_systems(FixedUpdate, fixed_update.run_if(in_state(MenuState::Title)));
    app.add_systems(
        Update,
        update.after(InputSet).run_if(in_state(MenuState::Title)),
    );
    app.add_systems(OnExit(MenuState::Title), on_exit);
}
