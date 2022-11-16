use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Piece;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Piece>()
        ;
    }
}

fn spawn_pieces(
    mut commands: Commands,
){
    
}