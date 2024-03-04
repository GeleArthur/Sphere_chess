mod types;
mod color;
mod position;
mod material;

use crate::chess::SelectedPiece;
use crate::chess::SelectedSquare;
use crate::pieces::types::*;
use crate::pieces::position::*;
pub use crate::pieces::material::*;
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
        .add_system(piece_position)
        .add_system(piece_selected)
        .add_system(selected_piece_update.after(piece_position).after(piece_selected));
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

    commands.spawn(MaterialMeshBundle{
        mesh,
        material,
        ..Default::default()
    })
    .insert(piece_type)
    .insert(color)
    .insert(position);
}

fn piece_selected(
    mut _commands: Commands,
    // world: &mut bevy::prelude::World,
    selected_piece: Query<Option<&SelectedPiece>>,
    mut pieces: Query<(&mut Transform, &PiecePosition, Entity)>,
    selected_position: Res<SelectedSquare>,
    buttons: Res<Input<MouseButton>>
){
    if selected_position.selecting_square == false {
        return;
    }

    for (mut transform, piece, entity) in &mut pieces {
        if selected_position.x == piece.x && selected_position.y == piece.y {
            let thing = transform.local_y() * 0.1;
            transform.translation += thing;

            if selected_piece.is_empty() == false {
                println!("EMPTY");
                return;
            }
            
            if buttons.just_released(MouseButton::Left){
                _commands.entity(entity).insert(SelectedPiece);
            }
        }        
    }
}

fn piece_position(
    mut _commands: Commands,
    mut pieces: Query<(&mut Transform, &PiecePosition)>,
    sphere: Query<&CenterSphere>,
) {
    for (mut transform, piece) in &mut pieces {
        let around = map(piece.x as f32, 0.0, 8.0, -PI, PI) + PI / 8.0;
        let updown = map(piece.y as f32, 0.0, 8.0, -PI / 2.0, PI / 2.0) + PI / 2.0 / 8.0;

        let position_ss = stacks_and_sectors_to_sphere_position(updown, around, sphere.single().radius);
        transform.translation = position_ss;

        transform.look_at(position_ss * 2.0, Vec3::Y);
        transform.rotation *= Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0);
    }
}

fn selected_piece_update(
    mut commands: Commands,
    mut selected_piece: Query<(&mut Transform, &mut PiecePosition, Entity), With<SelectedPiece>>,
    board: Query<&PiecePosition, Without<SelectedPiece>>,
    selected_square: Res<SelectedSquare>,
    buttons: Res<Input<MouseButton>>,
    sphere: Query<&CenterSphere>,
){
    for (mut transform, mut piece_position, entity) in &mut selected_piece {
        let around = map(selected_square.x as f32, 0.0, 8.0, -PI, PI) + PI / 8.0;
        let updown = map(selected_square.y as f32, 0.0, 8.0, -PI / 2.0, PI / 2.0) + PI / 2.0 / 8.0;

        let position_ss = stacks_and_sectors_to_sphere_position(updown, around, sphere.single().radius);
        transform.translation = position_ss;

        transform.look_at(position_ss * 2.0, Vec3::Y);
        transform.rotation *= Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0);

        if buttons.just_released(MouseButton::Left) && selected_square.selecting_square == true {
            
            for position in &board {
                if selected_square.x == position.x && selected_square.y == position.y {
                    return;
                }
            }

            piece_position.x = selected_square.x;
            piece_position.y = selected_square.y;
            commands.entity(entity).remove::<SelectedPiece>();
        }
    }
}


fn piece_location_solver(mut commands: Commands, piece_type: PieceTypes, PiecePosition { x, y }: PiecePosition){
    
}