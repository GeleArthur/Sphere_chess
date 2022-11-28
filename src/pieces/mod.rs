mod types;
mod color;
mod position;

use crate::pieces::types::*;
use crate::pieces::position::*;
use std::f32::consts::PI;
use bevy::prelude::*;

use crate::{chess::CenterSphere, game_assets::GameAssets, util::*};

use self::color::PieceColor;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<PiecePosition>()
        .register_type::<PieceTypes>()
        .add_startup_system(spawn_pieces)
        .add_system(piece_posistion);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    for x in 0..8 {
        spawn_piece(PiecePosition {x:x,y:6}, PieceTypes::Pawn, PieceColor::Black, &mut commands, &assets)
    }

    spawn_piece(PiecePosition {x:0,y:7}, PieceTypes::King, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:1,y:7}, PieceTypes::Queen, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:2,y:7}, PieceTypes::Bishop, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:3,y:7}, PieceTypes::Knight, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:4,y:7}, PieceTypes::Rook, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:5,y:7}, PieceTypes::Rook, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:6,y:7}, PieceTypes::Knight, PieceColor::Black, &mut commands, &assets);
    spawn_piece(PiecePosition {x:7,y:7}, PieceTypes::Bishop, PieceColor::Black, &mut commands, &assets);

    for x in 0..8 {
        spawn_piece(PiecePosition {x:x,y:1}, PieceTypes::Pawn, PieceColor::White, &mut commands, &assets)
    }

    spawn_piece(PiecePosition {x:0,y:0}, PieceTypes::King, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:1,y:0}, PieceTypes::Queen, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:2,y:0}, PieceTypes::Bishop, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:3,y:0}, PieceTypes::Knight, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:4,y:0}, PieceTypes::Rook, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:5,y:0}, PieceTypes::Rook, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:6,y:0}, PieceTypes::Knight, PieceColor::White, &mut commands, &assets);
    spawn_piece(PiecePosition {x:7,y:0}, PieceTypes::Bishop, PieceColor::White, &mut commands, &assets);
}

fn spawn_piece(position: PiecePosition, piece_type: PieceTypes, color: PieceColor, commands: &mut Commands, assets: &Res<GameAssets>){
    let mesh = match piece_type {
        PieceTypes::Pawn => assets.pawn.clone(),
        PieceTypes::Bishop => assets.bishop.clone(),
        PieceTypes::Knight => assets.knight.clone(),
        PieceTypes::Rook => assets.rook.clone(),
        PieceTypes::Queen => assets.queen.clone(),
        PieceTypes::King => assets.king.clone(),
    };

    let material = match color {
        PieceColor::White => assets.white_material.clone(),
        PieceColor::Black => assets.black_material.clone(),
    };


    commands.spawn(PbrBundle{
        mesh,
        material,
        ..Default::default()
    })
    .insert(piece_type)
    .insert(color)
    .insert(position);
}



fn piece_posistion(
    mut _commands: Commands,
    mut pieces: Query<(&mut Transform, &PiecePosition)>,
    sphere: Query<&CenterSphere>,
) {
    for (mut transform, piece) in &mut pieces {
        let around = map(piece.x as f32, 0.0, 8.0, -PI, PI) + PI / 8.0;
        let updown = map(piece.y as f32, 0.0, 8.0, -PI / 2.0, PI / 2.0) + PI / 2.0 / 8.0;

        let position_ss =
            stacks_and_sectors_to_sphere_position(updown, around, sphere.single().radius);
        transform.translation = position_ss;

        transform.look_at(position_ss * 2.0, Vec3::Y);
        transform.rotation *= Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0);
    }
}
