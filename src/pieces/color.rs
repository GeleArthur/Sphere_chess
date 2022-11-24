use bevy::prelude::*;

#[derive(Component, Reflect, Clone)]
pub enum PieceColor {
    White,
    Black
}

impl std::default::Default for PieceColor {
    fn default() -> Self {
        PieceColor::White
    }
}
