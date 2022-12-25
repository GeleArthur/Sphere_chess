
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef}, asset::AssetPath,
};

impl Material for ChessSphereMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path(AssetPath::from("chess_sphere.wgsl"))
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "b7b0f6e5-5ab9-4191-8317-cb91af729ceb"]
pub struct ChessSphereMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}