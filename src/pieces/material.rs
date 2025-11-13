use bevy::{
    asset::{AssetPath, uuid}, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef
};

impl Material for PiecesMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path(AssetPath::from("pieces.wgsl"))
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct PiecesMaterial {
    #[uniform(0)]
    pub base_color: Color,
    #[uniform(1)]
    pub base_color_outline: Color,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Option<Handle<Image>>,
}