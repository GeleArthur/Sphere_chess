use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef}, asset::AssetPath,
};

impl Material for PiecesMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path(AssetPath::from("pieces.wgsl"))
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "b7b0f6e5-5ab9-4191-8317-cb91af729deb"]
pub struct PiecesMaterial {
    #[uniform(0)]
    pub base_color: Color,
    #[uniform(1)]
    pub base_color_outline: Color,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Option<Handle<Image>>,
}