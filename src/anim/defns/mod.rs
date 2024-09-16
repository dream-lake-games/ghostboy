pub mod juice_;
pub use juice_::*;
pub mod gboy;
pub use gboy::*;
pub mod skelly_;
pub use skelly_::*;
pub mod tombstone_;
pub use tombstone_::*;

use super::anim_man::register_anim;
use super::*;

pub(super) struct AnimDefnsPlugin;
impl Plugin for AnimDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_anim::<GBoyAnim>(app);
        register_anim::<SkellyAnim>(app);
        register_anim::<TombstoneAnim>(app);
        // juice
        register_anim::<DashFadeAnim>(app);
        register_anim::<SmokeDown>(app);
        register_anim::<SmokeCirc>(app);
    }
}
