use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
pub struct ParallaxX {
    mult: f32,
    wrap_size: f32,
}
impl ParallaxX {
    pub fn new(mult: f32, wrap_size: f32) -> Self {
        Self { mult, wrap_size }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ParallaxY {
    mult: f32,
    wrap_size: f32,
}
impl ParallaxY {
    pub fn new(mult: f32, wrap_size: f32) -> Self {
        Self { mult, wrap_size }
    }
}

#[derive(Resource)]
struct LastCameraPos(Pos);

fn update_parallaxes(
    camera: Query<&Pos, (With<DynamicCamera>, Without<ParallaxX>, Without<ParallaxY>)>,
    mut last_camera_tran: ResMut<LastCameraPos>,
    mut pxs: Query<(&mut Transform, &ParallaxX), (Without<DynamicCamera>, Without<ParallaxY>)>,
    mut pys: Query<(&mut Transform, &ParallaxY), (Without<DynamicCamera>, Without<ParallaxY>)>,
) {
    let Ok(cam_pos) = camera.get_single() else {
        return;
    };
    let diff = last_camera_tran.0.as_vec2() - cam_pos.as_vec2();
    last_camera_tran.0 = cam_pos.clone();
    for (mut tran, px) in &mut pxs {
        tran.translation.x += diff.x * px.mult;
        while tran.translation.x.abs() > px.wrap_size / 2.0 + 0.00001 {
            tran.translation.x += tran.translation.x.signum() * -1.0 * px.wrap_size;
        }
    }
    for (mut tran, py) in &mut pys {
        tran.translation.y += diff.y * py.mult;
        while tran.translation.y.abs() > py.wrap_size / 2.0 + 0.00001 {
            tran.translation.y += tran.translation.y.signum() * -1.0 * py.wrap_size;
        }
    }
}

#[derive(Bundle)]
struct MountainBundle {
    name: Name,
    sprite: SpriteBundle,
    scale_mode: ImageScaleMode,
    px: ParallaxX,
    render_layers: RenderLayers,
}
impl MountainBundle {
    fn new(path: &str, mult: f32, zix: f32, ass: &Res<AssetServer>) -> Self {
        Self {
            name: Name::new("mountain"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(480.0 * 3.0, 144.0)),
                    ..default()
                },
                texture: ass.load(path.to_string()),
                transform: Transform::from_translation(Vec2::ZERO.extend(zix)),
                ..default()
            },
            scale_mode: ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            px: ParallaxX::new(mult, 480.0),
            render_layers: BgLayer::render_layers(),
        }
    }
}

fn startup_mountains(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn(MountainBundle::new(
        "environment/mountains_far.png",
        0.13,
        -102.0,
        &ass,
    ));
    commands.spawn(MountainBundle::new(
        "environment/mountains_mid.png",
        0.37,
        -101.0,
        &ass,
    ));
}

fn some_menu_force_move(
    meta_state: Res<State<MetaState>>,
    mut last_camera_tran: ResMut<LastCameraPos>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
    time: Res<Time>,
) {
    let cam_pos = cam_pos.single();
    if matches!(meta_state.get(), MetaState::Menu(_)) {
        *last_camera_tran = LastCameraPos(
            cam_pos
                .clone()
                .translated(-1.0 * FRAMERATE * time.delta_seconds() * Vec2::X),
        );
    }
}

pub(super) fn register_parallax(app: &mut App) {
    app.insert_resource(LastCameraPos(Pos::new(0.0, 0.0)));
    app.add_systems(Startup, startup_mountains);
    app.add_systems(
        PostUpdate,
        (some_menu_force_move, update_parallaxes).after(CameraSet),
    );
}
