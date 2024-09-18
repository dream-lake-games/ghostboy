use crate::prelude::*;

macro_rules! impl_core_computed_state {
    ($type:ty, $var:ident) => {
        impl ComputedStates for $type {
            type SourceStates = MetaState;
            fn compute(sources: Self::SourceStates) -> Option<Self> {
                match sources {
                    MetaState::$var(thing) => Some(thing),
                    _ => None,
                }
            }
        }
    };
}
impl_core_computed_state!(MenuState, Menu);
impl_core_computed_state!(LevelState, Level);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MetaStateKind {
    Menu,
    Level,
}
impl ComputedStates for MetaStateKind {
    type SourceStates = MetaState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            MetaState::Level(_) => Some(Self::Level),
            MetaState::Menu(_) => Some(Self::Menu),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum LevelStateKind {
    Loading,
    Spawning,
    Playing,
}
impl ComputedStates for LevelStateKind {
    type SourceStates = MetaState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            MetaState::Level(LevelState::Loading(_)) => Some(Self::Loading),
            MetaState::Level(LevelState::Spawning) => Some(Self::Spawning),
            MetaState::Level(LevelState::Playing) => Some(Self::Playing),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum PhysicsState {
    Inactive,
    Active,
}
impl ComputedStates for PhysicsState {
    type SourceStates = (MetaState, PauseState);

    fn compute(sources: (MetaState, PauseState)) -> Option<Self> {
        match sources {
            (MetaState::Level(LevelState::Spawning), PauseState::Unpaused) => Some(Self::Active),
            (MetaState::Level(LevelState::Playing), PauseState::Unpaused) => Some(Self::Active),
            _ => Some(Self::Inactive),
        }
    }
}

pub(super) fn register_computed(app: &mut App) {
    app.add_computed_state::<MenuState>();
    app.add_computed_state::<LevelState>();
    app.add_computed_state::<LevelStateKind>();
    app.add_computed_state::<PhysicsState>();
}
