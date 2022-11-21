use std::f32::consts::PI;

use bevy::prelude::{shape::UVSphere, *};

use crate::{chess::CenterSphere, game_assets::GameAssets, util::*};

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
}

impl std::default::Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn
    }
}

#[derive(Component)]
pub struct oritionCheck;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Piece>()
            .register_type::<PieceTypes>()
            .add_startup_system(spawn_pieces)
            .add_system(piece_posistion);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for m in 0..8 {
        for n in 0..8 {
            commands
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh: assets.pawn.clone(),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .insert(Piece { x: n, y: m })
                .insert(Name::new(format!("Piece X:{n} Y: {m}")));

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(UVSphere {
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
                    transform: Transform::from_scale(Vec3::ONE * 0.1),
                    ..Default::default()
                })
                .insert(oritionCheck);
        }
    }
}

fn piece_posistion(
    mut _commands: Commands,
    mut pieces: Query<(&mut Transform, &Piece), Without<oritionCheck>>,
    sphere: Query<&CenterSphere>,
    mut orition: Query<&mut Transform, With<oritionCheck>>,
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

    // let mut iter = orition.iter_mut();

    // for mut trans in &mut orition {
    //     trans.translation = Vec3::ONE;
    // }

    for (mut ori, piece) in orition.iter_mut().zip(pieces.iter()) {
        ori.translation = piece.0.translation;
    }
}

// thread 'main' panicked at 'error[B0001]: 
// Query<&mut bevy_transform::components::transform::Transform, bevy_ecs::query::filter::With<sphere_chess::piece::oritionCheck>> 
// in system sphere_chess::piece::piece_posistion 
// accesses component(s) 
// bevy_transform::components::transform::Transform in 
// a way that conflicts with a previous system parameter. 
// Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`.', C:\Users\Arthur\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_ecs-0.8.1\src\system\system_param.rs:205:5

