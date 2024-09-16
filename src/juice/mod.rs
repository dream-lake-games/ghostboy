use crate::prelude::*;

pub mod camera_shake;

pub use camera_shake::*;

pub(super) struct JuicePlugin;
impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        camera_shake::register_camera_shake(app);
    }
}
