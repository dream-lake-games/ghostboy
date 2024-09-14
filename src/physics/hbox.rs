use crate::prelude::*;

/// HBOX?????
#[derive(Clone, Debug, Reflect)]
pub struct Hbox {
    offset: Vec2,
    size: UVec2,
}
impl Hbox {
    pub fn new() -> Self {
        Self {
            offset: default(),
            size: default(),
        }
    }
    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset.x = x;
        self.offset.y = y;
        self
    }
    pub fn with_size(mut self, w: u32, h: u32) -> Self {
        self.size.x = w;
        self.size.y = h;
        self
    }
    pub fn translated(&self, x: f32, y: f32) -> Self {
        Self {
            offset: self.offset + Vec2::new(x, y),
            size: self.size.clone(), // Not strictly needed but nice for clarity
        }
    }

    impl_get_copy!(offset, Vec2);
    impl_get_copy!(size, UVec2);
}

// I don't care that this is super verbose, and maybe inefficient. I want it to be correct.
// Can performance engineer later if needed.
impl Hbox {
    /// Returns if the two hitboxes overlap
    pub fn overlaps_with(&self, rhs: &Self) -> bool {
        let fsize = self.size.as_vec2();
        let my_x_min = self.offset.x - fsize.x / 2.0;
        let my_x_max = self.offset.x + fsize.x / 2.0;
        let my_y_min = self.offset.y - fsize.y / 2.0;
        let my_y_max = self.offset.y + fsize.y / 2.0;

        let ofsize = rhs.size.as_vec2();
        let dont_overlap_x = (my_x_max <= rhs.offset.x - ofsize.x / 2.0)
            || (rhs.offset.x + ofsize.x / 2.0 <= my_x_min);
        let dont_overlap_y = (my_y_max <= rhs.offset.y - ofsize.y / 2.0)
            || (rhs.offset.y + ofsize.y / 2.0 <= my_y_min);

        let overlaps = !dont_overlap_x && !dont_overlap_y;
        overlaps
    }

    /// If the two hitboxes overlap, return the vec that you need to move self to get it out of rhs
    pub fn get_push_out(&self, rhs: &Self) -> Option<Vec2> {
        // Hear me out: this might not be that inefficient.
        // Almost everytime we call this it returns none. Better to use simpler logic to get quick no in usual case.
        if !self.overlaps_with(rhs) {
            return None;
        }

        let fsize = self.size.as_vec2();
        let my_x_min = self.offset.x - fsize.x / 2.0;
        let my_x_max = self.offset.x + fsize.x / 2.0;
        let my_y_min = self.offset.y - fsize.y / 2.0;
        let my_y_max = self.offset.y + fsize.y / 2.0;

        let ofsize = rhs.size.as_vec2();
        let ox_min = rhs.offset.x - ofsize.x / 2.0;
        let ox_max = rhs.offset.x + ofsize.x / 2.0;
        let oy_min = rhs.offset.y - ofsize.y / 2.0;
        let oy_max = rhs.offset.y + ofsize.y / 2.0;

        let needed_left_push = (ox_min - my_x_max).min(0.0);
        let needed_right_push = (ox_max - my_x_min).max(0.0);
        let needed_down_push = (oy_min - my_y_max).min(0.0);
        let needed_up_push = (oy_max - my_y_min).max(0.0);

        let needed_hor_push = if needed_left_push.abs() < needed_right_push.abs() {
            needed_left_push
        } else {
            needed_right_push
        };
        let needed_ver_push = if needed_down_push.abs() < needed_up_push.abs() {
            needed_down_push
        } else {
            needed_up_push
        };

        let push = if needed_hor_push.abs() < needed_ver_push.abs() {
            Vec2::new(needed_hor_push, 0.0)
        } else {
            Vec2::new(0.0, needed_ver_push)
        };

        Some(push)
    }
}
