pub mod anim;
pub mod camera;
pub mod consts;
pub mod debug;
pub mod enemies;
pub mod environment;
pub mod gboy;
pub mod input;
pub mod juice;
pub mod layer;
pub mod macros;
pub mod math;
pub mod menu;
pub mod my_ldtk;
pub mod physics;
pub mod roots;
pub mod state;
pub mod types;

pub mod prelude {
    pub use super::{
        anim::*, camera::*, consts::*, debug::*, enemies::*, environment::*, gboy::*, input::*,
        juice::*, layer::*, macros::*, menu::*, my_ldtk::*, physics::*, roots::*, state::*,
        types::*,
    };
    pub use bevy::{
        color::palettes::tailwind,
        ecs::component::StorageType,
        input::common_conditions::input_toggle_active,
        math::VectorSpace,
        prelude::*,
        reflect::GetTypeRegistration,
        render::view::RenderLayers,
        utils::{HashMap, HashSet},
    };
    pub use bevy_ecs_ldtk::ldtk::FieldInstance;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use rand::prelude::SliceRandom;
    pub use rand::{thread_rng, Rng};
    pub use std::time::Duration;
}
use prelude::*;

fn get_window_plugin() -> WindowPlugin {
    use bevy::window::*;
    #[allow(unexpected_cfgs)]
    if cfg!(target_arch = "wasm32") {
        WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                title: "GB TEMPLATE".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH_f32 * 3.0, SCREEN_HEIGHT_f32 * 3.0),
                ..default()
            }),
            ..default()
        }
    } else {
        WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                title: "GB TEMPLATE".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }
    }
}

fn main() {
    let mut app = App::new();

    // Bevy (or ecosystem) Plugins
    use bevy::asset::AssetMetaCheck;
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(get_window_plugin())
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)));

    // Our plugins
    app.add_plugins(AnimPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(GBoyPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(JuicePlugin)
        .add_plugins(LayerPlugin::new(SCREEN_UVEC))
        .add_plugins(MenuPlugin)
        .add_plugins(MyLdtkPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(RootPlugin)
        .add_plugins(StatePlugin);

    // Have fun!
    app.run();
}
