#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

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
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

    // let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, false);
    // let offset_and_counts = unpack_offset_and_counts(cluster_index);

    // // point lights
    // for (var i: u32 = offset_and_counts[0]; i < offset_and_counts[0] + offset_and_counts[1]; i = i + 1u) {
    //     let light_id = get_light_id(i);
    //     let light = point_lights.data[light_id];
    //     // var shadow: f32 = 1.0;
    //     // if ((mesh.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
    //     //         && (light.flags & POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
    //     //     shadow = fetch_point_shadow(light_id, in.world_position, in.world_normal);
    //     // }
    //     // let light_contrib = point_light(in.world_position.xyz, light, roughness, NdotV, in.N, in.V, R, F0, diffuse_color);
    //     // light_accum = light_accum + light_contrib * shadow;
    // }


    var lights = dot(in.world_normal.xyz, vec3<f32>(1.0,1.0,0.0));
    return vec4(lights * textureSample(base_color_texture, base_color_sampler, in.uv).rgb * material.color.rgb,1.0);
}
