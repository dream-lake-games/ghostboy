pub mod anim;
pub mod consts;
pub mod layer;
pub mod macros;
pub mod physics;
pub mod roots;
pub mod shorthand;
pub mod types;

pub mod prelude {
    pub use super::{
        anim::*, consts::*, layer::*, macros::*, physics::*, roots::*, shorthand::*, types::*,
    };
    pub use bevy::{
        input::common_conditions::input_toggle_active,
        prelude::*,
        reflect::GetTypeRegistration,
        render::view::RenderLayers,
        utils::{HashMap, HashSet},
    };
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use std::time::Duration;
}
use prelude::*;

fn play_startup(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), AnimMan::<SuicidoBody>::new()));
}

fn main() {
    let mut app = App::new();

    // Bevy (or ecosystem) Plugins
    use bevy::{asset::AssetMetaCheck, window::*};
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    title: "GB TEMPLATE".to_string(),
                    resolution: WindowResolution::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32),
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)));

    // Our plugins
    app.add_plugins(AnimPlugin)
        .add_plugins(LayerPlugin::new(SCREEN_UVEC))
        .add_plugins(PhysicsPlugin)
        .add_plugins(RootPlugin);

    // Play
    app.add_systems(Startup, play_startup);

    // Have fun!
    app.run();
}
