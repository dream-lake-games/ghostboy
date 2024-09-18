use crate::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct ShadeRemapMat {
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
    // The below need to be packed into Vec4 for wasm where stuff has to be 16-byte aligned
    #[uniform(3)]
    pub color1: Vec4,
    #[uniform(4)]
    pub color2: Vec4,
    #[uniform(5)]
    pub color3: Vec4,
    #[uniform(6)]
    pub color4: Vec4,
}
impl ShadeRemapMat {
    pub(super) fn new(
        texture: Handle<Image>,
        color1: Color,
        color2: Color,
        color3: Color,
        color4: Color,
    ) -> Self {
        Self {
            texture,
            color1: color_as_vec4(color1),
            color2: color_as_vec4(color2),
            color3: color_as_vec4(color3),
            color4: color_as_vec4(color4),
        }
    }
}

pub fn color_as_vec4(color: Color) -> Vec4 {
    let linear = color.to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, 1.0)
}

impl Material2d for ShadeRemapMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/shade_remap_mat.wgsl".into()
    }
}
