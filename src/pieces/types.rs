use bevy::prelude::*;

#[derive(Component, Reflect, Clone)]
pub enum PieceTypes {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl std::default::Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn
    }
}
