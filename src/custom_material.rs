
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef}, asset::AssetPath,
};

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path(AssetPath::from("custom_material.wgsl"))
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "b7b0f6e5-5ab9-4191-8317-cb91af729ceb"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}