#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var texture: texture_2d<f32>;
@group(2) @binding(2)
var splr: sampler;
@group(2) @binding(3)
var<uniform> color1: vec4<f32>;
@group(2) @binding(4)
var<uniform> color2: vec4<f32>;
@group(2) @binding(5)
var<uniform> color3: vec4<f32>;
@group(2) @binding(6)
var<uniform> color4: vec4<f32>;

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(texture, splr, in.uv);
    
    if original[0] < 0.2 {
        return to_linear(vec4<f32>(color4[0], color4[1], color4[2], 1.0));
    } else if original[0] < 0.3 {
        return to_linear(vec4<f32>(color3[0], color3[1], color3[2], 1.0));
    } else if original[0] < 0.5 {
        return to_linear(vec4<f32>(color2[0], color2[1], color2[2], 1.0));
    } else {
        return to_linear(vec4<f32>(color1[0], color1[1], color1[2], 1.0));
    }
}
