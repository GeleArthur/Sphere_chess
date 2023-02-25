#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#define_import_path bevy_pbr::pbr_functions

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var lights = max(dot(in.world_normal.xyz, vec3<f32>(1.0,1.0,0.0)),0.0);
    // lights += max(dot(in.world_normal.xyz, vec3<f32>(-1.0,-1.0,0.0)),0.0) * 0.01;


    return vec4(textureSample(base_color_texture, base_color_sampler, in.uv).rgb, 1.0);
}
