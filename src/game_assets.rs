use bevy::prelude::*;

use crate::custom_material::CustomMaterial;

#[derive(Resource)]
pub struct GameAssets {
    pub pawn: Handle<Mesh>,
    pub bishop: Handle<Mesh>,
    pub knight: Handle<Mesh>,
    pub rook: Handle<Mesh>,
    pub queen: Handle<Mesh>,
    pub king: Handle<Mesh>,

    pub white_material: Handle<StandardMaterial>,
    pub black_material: Handle<StandardMaterial>,

    pub board_material: Handle<CustomMaterial>,
}

pub fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_custom: ResMut<Assets<CustomMaterial>>,
) {
    commands.insert_resource(GameAssets {
        pawn: assets.load("pieces.glb#Mesh1/Primitive0"),
        bishop: assets.load("pieces.glb#Mesh4/Primitive0"),
        knight: assets.load("pieces.glb#Mesh2/Primitive0"),
        rook: assets.load("pieces.glb#Mesh3/Primitive0"),
        queen: assets.load("pieces.glb#Mesh5/Primitive0"),
        king: assets.load("pieces.glb#Mesh0/Primitive0"),

        black_material: materials.add(Color::rgba(0.0, 0.0, 0.0, 1.0).into()),
        white_material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),

        board_material: materials_custom.add(CustomMaterial {
            color: Color::BLUE,
            color_texture: Some(assets.load("board.png")),
            alpha_mode: AlphaMode::Blend,
        }),
    })
}
