use bevy::prelude::*;

pub struct GameAssets {
    pub pawn: Handle<Mesh>,
    pub bishop: Handle<Mesh>,
    pub knight: Handle<Mesh>,
    pub rook: Handle<Mesh>,
    pub queen: Handle<Mesh>,
    pub king: Handle<Mesh>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        pawn: assets.load("pieces.glb#Mesh0/Primitive0"),
        bishop: assets.load("pieces.glb#Mesh0/Primitive0"),
        knight: assets.load("pieces.glb#Mesh0/Primitive0"),
        rook: assets.load("pieces.glb#Mesh0/Primitive0"),
        queen: assets.load("pieces.glb#Mesh0/Primitive0"),
        king: assets.load("pieces.glb#Mesh0/Primitive0"),
    })
}