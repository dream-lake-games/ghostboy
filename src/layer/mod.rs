use crate::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::{
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};
use bevy::sprite::{Material2dPlugin, Mesh2dHandle};
use bevy::window::WindowResized;
use shade_remap_mat::{color_as_vec4, ShadeRemapMat};

mod shade_remap_mat;

pub trait CameraLayer {
    const _RENDER_LAYER: usize;

    fn layer() -> usize {
        Self::_RENDER_LAYER
    }

    fn render_layers() -> RenderLayers {
        RenderLayers::from_layers(&[Self::_RENDER_LAYER])
    }
}

macro_rules! decl_layer {
    ($name:ident, $order:literal) => {
        #[derive(Component, Debug, Reflect, Default)]
        pub struct $name;
        impl CameraLayer for $name {
            const _RENDER_LAYER: usize = $order;
        }
    };
}
decl_layer!(BgLayer, 1);
decl_layer!(MainLayer, 2);
decl_layer!(FgLayer, 3);
decl_layer!(MenuLayer, 4);

/// Grows all of the layers by a given scale.
/// Makes it easy for the game to fill the screen in a satisfying way.
#[derive(Resource, Clone, Copy)]
pub struct LayerGrowth {
    scale: f32,
}
impl LayerGrowth {
    impl_get_copy!(scale, f32);
    impl_set!(scale, f32);
}
impl Default for LayerGrowth {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

/// Configures the clear colors and ambient light of the layers.
#[derive(Debug, Resource, Clone)]
pub struct LayerClearColors {
    bg_clear_color: ClearColorConfig,
    main_clear_color: ClearColorConfig,
    fg_clear_color: ClearColorConfig,
    menu_clear_color: ClearColorConfig,
}
macro_rules! impl_clear_color_config_field {
    ($name:ident) => {
        impl_get_copy!($name, ClearColorConfig);
        impl_set!($name, ClearColorConfig);
        impl_with!($name, ClearColorConfig);
    };
}
impl LayerClearColors {
    impl_clear_color_config_field!(bg_clear_color);
    impl_clear_color_config_field!(main_clear_color);
    impl_clear_color_config_field!(fg_clear_color);
    impl_clear_color_config_field!(menu_clear_color);
}
impl Default for LayerClearColors {
    fn default() -> Self {
        Self {
            bg_clear_color: ClearColorConfig::Custom(COLOR_4),
            main_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            fg_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            menu_clear_color: ClearColorConfig::Custom(COLOR_NONE),
        }
    }
}

const BG_IMAGE: Handle<Image> = Handle::weak_from_u128(84562364042238462870);
const MAIN_IMAGE: Handle<Image> = Handle::weak_from_u128(64462261242435462111);
const FG_IMAGE: Handle<Image> = Handle::weak_from_u128(53466206864860343678);
const MENU_IMAGE: Handle<Image> = Handle::weak_from_u128(36467206864860383190);
const SHADE_REMAP_IMAGE: Handle<Image> = Handle::weak_from_u128(84732874238929384748);

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct ShadeRemaps {
    pub map: QColorMap,
}
impl ShadeRemaps {
    pub fn set(&mut self, base: QColor, to: QColor) {
        self.map.set(base, to);
    }
}

#[derive(Debug, Resource)]
struct CameraTargets {
    screen_size: UVec2,
    bg_target: Handle<Image>,
    main_target: Handle<Image>,
    fg_target: Handle<Image>,
    menu_target: Handle<Image>,
    shade_remap_target: Handle<Image>,
}
impl Default for CameraTargets {
    fn default() -> Self {
        Self {
            screen_size: SCREEN_UVEC,
            bg_target: default(),
            main_target: default(),
            fg_target: default(),
            menu_target: default(),
            shade_remap_target: default(),
        }
    }
}
impl CameraTargets {
    /// Creates actually images that the various layers can write to to place on quads.
    pub fn initialize(&mut self, images: &mut Assets<Image>) {
        macro_rules! make_layer_image {
            ($label:expr, $handle:expr) => {{
                let target_extent = Extent3d {
                    width: self.screen_size.x,
                    height: self.screen_size.y,
                    ..default()
                };

                // Makes the image
                let mut image = Image {
                    texture_descriptor: TextureDescriptor {
                        label: Some($label),
                        size: target_extent,
                        dimension: TextureDimension::D2,
                        format: TextureFormat::bevy_default(),
                        mip_level_count: 1,
                        sample_count: 1,
                        usage: TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_DST
                            | TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    },
                    ..default()
                };
                // Fills it with zeros
                image.resize(target_extent);
                images.insert($handle.id(), image);
                $handle
            }};
        }

        self.bg_target = make_layer_image!("bg_target", BG_IMAGE);
        self.main_target = make_layer_image!("main_target", MAIN_IMAGE);
        self.fg_target = make_layer_image!("fg_target", FG_IMAGE);
        self.menu_target = make_layer_image!("menu_target", MENU_IMAGE);
        self.shade_remap_target = make_layer_image!("shade_remap_target", SHADE_REMAP_IMAGE);
    }
}

fn setup_layer_materials(
    root: Res<LayerRoot>,
    mut commands: Commands,
    mut camera_targets: ResMut<CameraTargets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shade_remap_mats: ResMut<Assets<ShadeRemapMat>>,
) {
    camera_targets.initialize(&mut images);
    let actual_stuff_layer = RenderLayers::from_layers(&[29]);
    let shade_remap_layer = RenderLayers::from_layers(&[30]);

    macro_rules! setup_layer {
        ($name:literal, $image:expr, $zix:literal) => {
            commands
                .spawn((
                    Name::new($name),
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::Z * $zix as f32),
                        texture: $image,
                        ..default()
                    },
                    // ResizeImage,
                    actual_stuff_layer.clone(),
                ))
                .set_parent(root.eid());
        };
    }
    setup_layer!("bg_image", BG_IMAGE, 0);
    setup_layer!("main_image", MAIN_IMAGE, 1);
    setup_layer!("fg_image", FG_IMAGE, 2);
    setup_layer!("menu_image", MENU_IMAGE, 3);

    // This is the camera that sees all of the layer quads and squashes them into one image
    commands
        .spawn((
            Name::new("final_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 6,
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    target: RenderTarget::Image(SHADE_REMAP_IMAGE),
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            actual_stuff_layer,
        ))
        .set_parent(root.eid());

    // This is the quad with the shade remaps
    let mesh = Mesh::from(Rectangle::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32));
    let mesh_2d: Mesh2dHandle = meshes.add(mesh).into();
    let mat = ShadeRemapMat::new(SHADE_REMAP_IMAGE, COLOR_1, COLOR_2, COLOR_3, COLOR_4);
    commands
        .spawn((
            Name::new("shade_remaped"),
            // SpriteBundle {
            //     texture: SHADE_REMAP_IMAGE,
            //     ..default()
            // },
            mesh_2d,
            shade_remap_mats.add(mat),
            SpatialBundle::default(),
            ResizeImage,
            shade_remap_layer.clone(),
        ))
        .set_parent(root.eid());

    // This is currently the final camera
    commands
        .spawn((
            Name::new("final_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 7,
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            shade_remap_layer,
        ))
        .set_parent(root.eid());
}

fn setup_layer_cameras(
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    layer_colors: Res<LayerClearColors>,
    layer_root: Res<LayerRoot>,
) {
    macro_rules! spawn_layer_camera {
        ($comp:ty, $name:expr, $order:expr, $image:expr, $clear_color:expr, $follow_dynamic:expr) => {{
            let mut comms = commands.spawn((
                Name::new($name),
                Camera2dBundle {
                    camera: Camera {
                        order: $order,
                        target: RenderTarget::Image($image),
                        clear_color: $clear_color,
                        ..default()
                    },
                    projection: OrthographicProjection {
                        near: ZIX_MIN,
                        far: ZIX_MAX,
                        scale: 1.0,
                        ..default()
                    },
                    ..default()
                },
                <$comp>::default(),
                <$comp>::render_layers(),
            ));
            comms.set_parent(layer_root.eid());
            if $follow_dynamic {
                comms.insert(FollowDynamicCamera);
            }
        }};
    }
    spawn_layer_camera!(
        BgLayer,
        "bg_camera",
        0,
        camera_targets.bg_target.clone(),
        layer_colors.bg_clear_color,
        false
    );
    spawn_layer_camera!(
        MainLayer,
        "main_camera",
        1,
        camera_targets.main_target.clone(),
        layer_colors.main_clear_color,
        true
    );
    spawn_layer_camera!(
        FgLayer,
        "fg_camera",
        2,
        camera_targets.fg_target.clone(),
        layer_colors.fg_clear_color,
        false
    );
    spawn_layer_camera!(
        MenuLayer,
        "menu_camera",
        2,
        camera_targets.menu_target.clone(),
        layer_colors.menu_clear_color,
        false
    );
}

/// Marks the layer images which should respond to resizing events
#[derive(Component)]
struct ResizeImage;

/// After resizing to fill the screen (as best we can), how many mults over screen size did we go?
#[derive(Resource, Debug, Reflect)]
pub struct OverScreenMult(pub f32);

fn resize_canvases(
    mut events: EventReader<WindowResized>,
    mut quad_trans: Query<&mut Transform, With<ResizeImage>>,
    mut over_screen_mult: ResMut<OverScreenMult>,
) {
    let Some(event) = events.read().last() else {
        return;
    };

    // Mult is smallest to fill either vertically or horizontally
    // A.k.a don't cut anything off
    let width_mult = event.width / SCREEN_WIDTH_f32;
    let height_mult = event.height / SCREEN_HEIGHT_f32;
    let mult = width_mult.min(height_mult);
    over_screen_mult.0 = mult;

    // Then update the layering quads
    for mut tran in &mut quad_trans {
        tran.scale = (Vec2::ONE * mult).extend(1.0);
    }
}

fn update_shade_remaps(
    remap_res: Res<ShadeRemaps>,
    hands: Query<&Handle<ShadeRemapMat>>,
    mut mats: ResMut<Assets<ShadeRemapMat>>,
) {
    for hand in &hands {
        let Some(mat) = mats.get_mut(hand.id()) else {
            continue;
        };
        mat.color1 = color_as_vec4(remap_res.map.get(QColor::Color1).to_actual_color());
        mat.color2 = color_as_vec4(remap_res.map.get(QColor::Color2).to_actual_color());
        mat.color3 = color_as_vec4(remap_res.map.get(QColor::Color3).to_actual_color());
        mat.color4 = color_as_vec4(remap_res.map.get(QColor::Color4).to_actual_color());
    }
}

#[derive(Default)]
pub struct LayerPlugin {
    screen_size: UVec2,
    layer_clear_colors: LayerClearColors,
    layer_growth: LayerGrowth,
}
impl LayerPlugin {
    pub fn new(screen_size: UVec2) -> Self {
        Self {
            screen_size,
            ..default()
        }
    }

    impl_get_ref!(layer_clear_colors, LayerClearColors);
    impl_set!(layer_clear_colors, LayerClearColors);
    impl_with!(layer_clear_colors, LayerClearColors);

    impl_get_copy!(layer_growth, LayerGrowth);
    impl_set!(layer_growth, LayerGrowth);
}
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.layer_clear_colors.clone());
        app.insert_resource(self.layer_growth.clone());
        app.insert_resource(ShadeRemaps { map: default() });
        let cam_targets = CameraTargets {
            screen_size: self.screen_size,
            ..default()
        };
        app.insert_resource(cam_targets);
        app.insert_resource(OverScreenMult(1.0));
        app.add_plugins(Material2dPlugin::<shade_remap_mat::ShadeRemapMat>::default());

        app.add_systems(
            Startup,
            (setup_layer_materials, setup_layer_cameras)
                .chain()
                .after(RootInit),
        );
        app.add_systems(Update, (resize_canvases, update_shade_remaps));
    }
}
