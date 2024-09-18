use crate::prelude::*;

mod camera_movement;

pub use camera_movement::{DynamicCamera, DynamicCameraMode, FollowDynamicCamera};

/// The set that contains all camera related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

pub(super) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        camera_movement::register_camera_movement(app);
    }
}
