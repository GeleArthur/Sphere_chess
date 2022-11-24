use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PiecePosition {
    pub x: i8,
    pub y: i8,
}