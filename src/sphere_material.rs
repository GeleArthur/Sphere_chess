
use bevy::{
    asset::AssetPath, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef
};

impl Material for ChessSphereMaterial {
    fn fragment_shader() -> ShaderRef {
        "chess_sphere.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChessSphereMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}