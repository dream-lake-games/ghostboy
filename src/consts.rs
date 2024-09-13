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
    red: 0.91372549019,
    green: 0.96078431372,
    blue: 0.85490196078,
    alpha: 1.0,
});
pub const COLOR_2: Color = Color::Srgba(Srgba {
    red: 0.94117647058,
    green: 0.71372549019,
    blue: 0.58431372549,
    alpha: 1.0,
});
pub const COLOR_3: Color = Color::Srgba(Srgba {
    red: 0.5294117647,
    green: 0.44705882352,
    blue: 0.52549019607,
    alpha: 1.0,
});
pub const COLOR_4: Color = Color::Srgba(Srgba {
    red: 0.2431372549,
    green: 0.22745098039,
    blue: 0.25882352941,
    alpha: 1.0,
});
