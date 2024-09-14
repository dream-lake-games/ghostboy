use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
pub struct Dyno {
    vel: Vec2,
}

pub(super) fn register_dyno(app: &mut App) {}
