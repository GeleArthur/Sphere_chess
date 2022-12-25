use bevy::prelude::*;

use crate::custom_material::ChessSphereMaterial;

#[derive(Resource)]
pub struct GameAssets {
    pub pawn: Handle<Mesh>,
    pub bishop: Handle<Mesh>,
    pub knight: Handle<Mesh>,
    pub rook: Handle<Mesh>,
    pub queen: Handle<Mesh>,
    pub king: Handle<Mesh>,

    pub white_material: Handle<ChessSphereMaterial>,
    pub black_material: Handle<ChessSphereMaterial>,

    pub board_material: Handle<ChessSphereMaterial>,
}

pub fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials_custom: ResMut<Assets<ChessSphereMaterial>>
) {
    commands.insert_resource(GameAssets {
        pawn: assets.load("pieces.glb#Mesh1/Primitive0"),
        bishop: assets.load("pieces.glb#Mesh4/Primitive0"),
        knight: assets.load("pieces.glb#Mesh2/Primitive0"),
        rook: assets.load("pieces.glb#Mesh3/Primitive0"),
        queen: assets.load("pieces.glb#Mesh5/Primitive0"),
        king: assets.load("pieces.glb#Mesh0/Primitive0"),

        black_material: materials_custom.add(ChessSphereMaterial {
            color: Color::rgb(0.1,0.1,0.1),
            color_texture: Some(assets.load("white.png")),
        }),

        white_material: materials_custom.add(ChessSphereMaterial {
            color: Color::WHITE,
            color_texture: Some(assets.load("white.png")),
        }),

        board_material: materials_custom.add(ChessSphereMaterial {
            color: Color::WHITE,
            color_texture: Some(assets.load("board.png")),
        }),
    })
}