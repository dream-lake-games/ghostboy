use crate::prelude::*;

macro_rules! defn_effects {
    ([$($name:ident, $path:literal, $mult:literal,)*]) => {
        #[derive(Component, Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
        pub enum SoundEffect {
            $($name,)*
        }
        impl SoundEffect {
            pub fn path(&self) -> String {
                match self {
                    $(Self::$name => $path.to_string(),)*
                }
            }
            pub fn mult(&self) -> f32 {
                match self {
                    $(Self::$name => $mult,)*
                }
            }
        }

        #[derive(Resource, Reflect)]
        pub struct SoundMults {
            pub map: HashMap<SoundEffect, f32>,
        }
        impl Default for SoundMults {
            fn default() -> Self {
                let mut map = HashMap::new();
                $(
                    map.insert(SoundEffect::$name, $mult);
                )*
                Self { map }
            }
        }
    };
}

defn_effects!([
    Death1,
    "sound_effects/control/death1.ogg",
    0.1,
    Death2,
    "sound_effects/control/death2.ogg",
    0.14,
    Death3,
    "sound_effects/control/death3.ogg",
    0.2,
    ReachSpawn,
    "sound_effects/control/reach_spawn.ogg",
    0.05,
    Replenish,
    "sound_effects/control/replenish.ogg",
    0.05,
    Thunder,
    "sound_effects/control/thunder.ogg",
    0.06,
    LightRain,
    "sound_effects/environment/lightrain.ogg",
    0.2,
    Normal1,
    "sound_effects/impact/normal1.ogg",
    0.08,
    Normal2,
    "sound_effects/impact/normal2.ogg",
    0.08,
    Normal3,
    "sound_effects/impact/normal3.ogg",
    0.08,
    PauseIn,
    "sound_effects/menu/pausein1.ogg",
    0.2,
    PauseOut,
    "sound_effects/menu/pauseout1.ogg",
    0.2,
    Select,
    "sound_effects/menu/select1.ogg",
    0.2,
]);

pub(super) fn register_effect_defns(app: &mut App) {
    app.insert_resource(SoundMults::default());
    debug_resource!(app, SoundMults);
}
