use bevy::sprite::Material2dPlugin;

use crate::prelude::*;

pub mod anim_macros;
mod anim_man;
pub mod anim_mat;
pub mod defns;

pub use anim_macros::*;
pub use anim_man::{
    AnimBody, AnimBodyProgress, AnimMan, AnimStateMachine, MutableAnimationManagerActions,
};
use anim_man::{AnimBodyData, AnimBodyDataOverrides, AnimNextState, AnimStateData};
pub use anim_mat::*;
pub use defns::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ManagersSet;

pub(super) struct AnimPlugin;
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<anim_mat::AnimMat>();
        app.register_type::<anim_man::AnimBodyData>();

        app.add_plugins(Material2dPlugin::<anim_mat::AnimMat>::default());
        app.add_plugins(defns::AnimDefnsPlugin);
    }
}
