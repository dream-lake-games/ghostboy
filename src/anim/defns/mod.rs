pub mod gboy;
pub use gboy::*;

use super::anim_man::register_anim;
use super::*;

pub(super) struct AnimDefnsPlugin;
impl Plugin for AnimDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_anim::<GBoyAnim>(app);
    }
}
