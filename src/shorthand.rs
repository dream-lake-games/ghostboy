//! Things that solely exist to reduce keystrokes, but ARE NOT macros

use crate::prelude::*;

pub fn pos_zix_to_spat(pos: Pos, zix: Zix) -> SpatialBundle {
    SpatialBundle::from_transform(Transform::from_translation(pos.extend(zix).as_vec3()))
}

pub fn place_to_spat(place: Place) -> SpatialBundle {
    pos_zix_to_spat(place.pos, place.zix)
}
