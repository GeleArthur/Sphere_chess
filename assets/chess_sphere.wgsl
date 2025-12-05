#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
    pbr_functions as fns,
    pbr_bindings,
}

// #import bevy_pbr::clustered_forward
// #import bevy_pbr::lighting
// #import bevy_pbr::shadows

struct CustomMaterial {
    color: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: CustomMaterial;
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var lights = max(dot(in.world_normal.xyz, vec3<f32>(0.71,0.71,0.0)),0.0);
    // lights += max(dot(in.world_normal.xyz, vec3<f32>(-1.0,-1.0,0.0)),0.0) * 0.01;


    return vec4(textureSample(base_color_texture, base_color_sampler, in.uv).rgb , 1.0);
}
