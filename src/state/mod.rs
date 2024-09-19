use crate::prelude::*;

pub mod computed;

pub use computed::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum MenuState {
    Title,
    WorldSelect,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum LevelState {
    Loading(LevelLoadingState),
    Spawning,
    Playing,
    Dying,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub struct LevelLoadingState {
    pub world_path: String,
    pub level_iid: LevelIid,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)]
pub enum MetaState {
    Menu(MenuState),
    Level(LevelState),
}

/// Kills some verbosity with reading meta states
pub trait MetaUnfucker {
    fn get_menu_state(&self) -> Option<MenuState>;
    fn get_level_state(&self) -> Option<LevelState>;
}
impl MetaUnfucker for MetaState {
    fn get_menu_state(&self) -> Option<MenuState> {
        match self {
            MetaState::Menu(menu_state) => Some(menu_state.clone()),
            _ => None,
        }
    }

    fn get_level_state(&self) -> Option<LevelState> {
        match self {
            MetaState::Level(level_state) => Some(level_state.clone()),
            _ => None,
        }
    }
}
impl MetaUnfucker for State<MetaState> {
    fn get_menu_state(&self) -> Option<MenuState> {
        MetaState::get_menu_state(self.get())
    }
    fn get_level_state(&self) -> Option<LevelState> {
        MetaState::get_level_state(self.get())
    }
}

/// Kills some verbosity for writing meta states
pub trait ToMetaState {
    fn to_meta_state(&self) -> MetaState;
}
macro_rules! impl_to_meta_state {
    ($type:ty, $disc:ident) => {
        impl ToMetaState for $type {
            fn to_meta_state(&self) -> MetaState {
                MetaState::$disc(self.clone())
            }
        }
    };
}
impl_to_meta_state!(MenuState, Menu);
impl_to_meta_state!(LevelState, Level);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum PauseState {
    Unpaused,
    Paused,
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MenuState::Title.to_meta_state());
        // app.insert_state(
        //     LevelState::Loading(LevelLoadingState {
        //         world_path: "ldtk/world.ldtk".to_string(),
        //         level_iid: LevelIid::new("6707e010-4ce0-11ef-8458-1d8de6fabb3d".to_string()),
        //     })
        //     .to_meta_state(),
        // );
        app.insert_state(PauseState::Unpaused);
        computed::register_computed(app);
    }
}
