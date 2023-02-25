#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// #import bevy_pbr::utils
// #import bevy_pbr::clustered_forward
// #import bevy_pbr::lighting
// #import bevy_pbr::shadows

struct CustomMaterial {
    base_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> base_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> outline_color: vec4<f32>;
@group(1) @binding(2)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var base_color_sampler: sampler;

fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2:f32) -> f32{
    return (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
}

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var normal = normalize(world_normal);
    var viewVector = normalize(view.world_position - world_normal.xyz);

    var angleBetween = dot(normal, viewVector);
    var fresnel = max(angleBetween, 0.0);


    fresnel = smoothstep(0.3, 0.0, fresnel);

    var color = base_color;
    color = mix(color, outline_color, fresnel);

    var lights = dot(world_normal.xyz, vec3<f32>(1.0,1.0,0.0));

    // var colorInverted = vec3(1.0) - material.color.rgb;

    return vec4(color);
}