use crate::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct AnimMat {
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
    // The below need to be packed into Vec4 for wasm where stuff has to be 16-byte aligned
    #[uniform(3)]
    ix_length_flipx_flipy: Vec4, // NOTE: 1.0 = don't flip, -1.0 = flip
    #[uniform(4)]
    xoff_yoff_xrep_yrep: Vec4,
    #[uniform(5)]
    rgba: Vec4,
}
impl AnimMat {
    const fn flip_to_mul(val: bool) -> f32 {
        if val {
            -1.0
        } else {
            1.0
        }
    }

    pub(super) fn new(
        texture: Handle<Image>,
        length: u32,
        flip_x: bool,
        flip_y: bool,
        repetitions: IVec2,
    ) -> Self {
        let srgba_thing = Srgba::rgb_u8(255, 255, 255);
        Self {
            texture,
            ix_length_flipx_flipy: Vec4::new(
                0.0,
                length as f32,
                Self::flip_to_mul(flip_x),
                Self::flip_to_mul(flip_y),
            ),
            xoff_yoff_xrep_yrep: Vec4::new(0.0, 0.0, repetitions.x as f32, repetitions.y as f32),
            rgba: Vec4::new(
                srgba_thing.red,
                srgba_thing.green,
                srgba_thing.blue,
                srgba_thing.alpha,
            ),
        }
    }

    pub(super) fn set_ix(&mut self, ix: u32) {
        self.ix_length_flipx_flipy[0] = ix as f32;
    }
}

impl Material2d for AnimMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/animation_mat.wgsl".into()
    }
}
