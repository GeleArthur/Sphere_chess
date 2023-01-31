#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// #import bevy_pbr::utils
// #import bevy_pbr::clustered_forward
// #import bevy_pbr::lighting
// #import bevy_pbr::shadows

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
    
    @location(0) vertex_position: vec4<f32>,
    @location(1) vertex_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    // @location(3) world_tangent: vec4<f32>,
    // @location(4) color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    //var n = (view.view[0][0] * in.vertex_normal)

    let myMat3x3 = mat3x3(view.view[0].xyz, view.view[1].xyz, view.view[2].xyz);

    var n = normalize(in.vertex_normal * myMat3x3);
    var p = (view.view * in.vertex_position).xyz;
    var v = normalize(-p);
    var vdn = dot(p,n);
    //var toEye = normalize(in.world_position.xyz - view.world_position.xyz);
    //var diffrents = vec3(dot(normalize(in.world_normal.xyz), toEye));
    //var edge = smoothstep(0.1, 1.0, vdn);

    //var color = vec4(1.0);

    return vec4(vdn,vdn,vdn,1.0);
    // return vec4(toEye * textureSample(base_color_texture, base_color_sampler, in.uv).rgb * material.color.rgb,1.0);
}
