use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{game_assets::GameAssets, util::*, chess::CenterSphere};

#[derive(Component, Reflect, Clone)]
pub enum PieceTypes {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Piece {
    x: i8,
    y: i8,
    // piece_type: PieceTypes,
    killme: f32,
    killme2: f32,
    killme3: f32,
}

impl std::default::Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn
    }
}




pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Piece>()
        .register_type::<PieceTypes>()
        .add_startup_system(spawn_pieces)
        .add_system(piece_posistion);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for m in 0..8{
        for n in 0..8 {
            commands.spawn().insert_bundle(PbrBundle{
                mesh: assets.pawn.clone(),
                material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
                ..Default::default()
            }).insert(Piece{x:n,y:m, killme: 0.0, killme2:0.0, killme3:0.0});
        }
    }
}

fn piece_posistion(mut commands: Commands, mut pieces: Query<(&mut Transform, &Piece)>, sphere: Query<&CenterSphere>) {
    for (mut transform, piece)in &mut pieces {
        let around = map(piece.x as f32, 0.0, 8.0, -PI, PI) + PI/8.0;
        let updown = map(piece.y as f32, 0.0, 8.0, -PI/2.0, PI/2.0,) + PI/2.0/8.0;

        let position_ss = stacks_and_sectors_to_sphere_position(updown, around, sphere.single().radius);
        transform.translation = position_ss;

        transform.look_at(position_ss*2.0, Vec3::Y);
        transform.rotation *= Quat::from_euler(EulerRot::XYZ, -PI/2.0, 0.0, 0.0);

    }
}
