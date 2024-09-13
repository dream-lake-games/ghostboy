pub mod example;
use super::anim_man::register_anim;
use super::*;
pub use example::*;

pub(super) struct AnimDefnsPlugin;
impl Plugin for AnimDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_anim::<SuicidoBody>(app);
    }
}
