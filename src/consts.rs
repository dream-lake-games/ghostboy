use crate::prelude::*;

// Often nice to have the size of the screen in these formats
pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCREEN_UVEC: UVec2 = UVec2::new(SCREEN_WIDTH, SCREEN_HEIGHT);
#[allow(nonstandard_style)]
pub const SCREEN_WIDTH_f32: f32 = SCREEN_WIDTH as f32;
#[allow(nonstandard_style)]
pub const SCREEN_HEIGHT_f32: f32 = SCREEN_HEIGHT as f32;
pub const SCREEN_VEC: Vec2 = Vec2::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32);

// This is frames per second by the way
pub const FRAMERATE: f32 = 36.0;
pub const DEFAULT_ANIMATION_FPS: f32 = 16.0;

// Keeping constant ZIX's here makes debugging weird layering stuff MUCH easier
pub const ZIX_GBOY: f32 = 20.0;
pub const ZIX_SKELLY: f32 = 19.0;
pub const ZIX_MAX: f32 = 600.0; // Anything further forward than this gets culled by the camera(s)
pub const ZIX_MENU: i32 = 50;
pub const ZIX_MIN: f32 = -600.0; // Anything further back than this gets culled by the camera(s)
pub const ZIX_PARTICLES: i32 = 40;
pub const ZIX_TRANSITION: i32 = 60;

// Colors are easy!
pub const COLOR_NONE: Color = Color::Srgba(Srgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
});
pub const COLOR_1: Color = Color::Srgba(Srgba {
    red: 238.0 / 255.0,
    green: 191.0 / 255.0,
    blue: 245.0 / 255.0,
    alpha: 1.0,
});
pub const COLOR_2: Color = Color::Srgba(Srgba {
    red: 158.0 / 255.0,
    green: 129.0 / 255.0,
    blue: 208.0 / 255.0,
    alpha: 1.0,
});
pub const COLOR_3: Color = Color::Srgba(Srgba {
    red: 133.0 / 255.0,
    green: 69.0 / 255.0,
    blue: 118.0 / 255.0,
    alpha: 1.0,
});
pub const COLOR_4: Color = Color::Srgba(Srgba {
    red: 48.0 / 255.0,
    green: 18.0 / 255.0,
    blue: 33.0 / 255.0,
    alpha: 1.0,
});
