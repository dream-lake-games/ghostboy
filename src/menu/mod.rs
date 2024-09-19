use crate::prelude::*;

mod pause;
mod title;
mod world_select;

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        title::register_title(app);
        world_select::register_world_select(app);
        pause::register_pause(app);
    }
}
