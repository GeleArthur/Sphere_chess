use bevy::prelude::*;
use bevy::prelude::shape::UVSphere;

use crate::*;
use crate::constant::*;
use crate::game_assets::GameAssets;
use crate::util::*;


#[derive(Default, Resource)]
pub struct SelectedSquare {
    pub x: i8,
    pub y: i8
}

#[derive(Component)]
pub struct SelectedPiece;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CenterSphere {
    pub center: Vec3,
    pub radius: f32,
}

#[derive(Component, Default)]
pub struct ChessBoard {
    pub grid: [[Option<Entity>; 8]; 8],
}
pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<SelectedSquare>()
        .add_startup_system(spawn_board.label(BOARD_LABEL))
        .add_system(camera_sphere_select)
        ;
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>
) {
    commands
        .spawn(CenterSphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        })
        .insert(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(UVSphere {
                radius: 1.0,
                sectors: 8*4,
                stacks: 8*4,
                ..Default::default()
            })),
            material: assets.board_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::PI / 2.0,
                0.0,
                0.0,
            )),
            ..Default::default()
        })
        .insert(Name::new("Sphere"));
}

fn camera_sphere_select(
    camera: Query<(&Camera, &GlobalTransform)>,
    mut gizmos_cube: Query<&mut Transform, With<GizmosCube>>,
    sphere: Query<&CenterSphere>,
    mut selected: ResMut<SelectedSquare>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(mouse_position) = window.cursor_position() {
        let (camera, camera_transform) = camera.single();

        let raycasted = camera.viewport_to_world(camera_transform, mouse_position).unwrap();
        let mut giz_transform = gizmos_cube.single_mut();

        let sphere = sphere.single();

        let ray_hit = raycast_ball(raycasted, sphere.center, sphere.radius);

        if ray_hit.is_none() {
            return;
        }

        let hit = ray_hit.unwrap();

        let (stack,sector) = sphere_position_to_stacks_and_sectors(hit, sphere.radius);

        // lazy mode
        let around = util::map(sector, -std::f32::consts::PI, std::f32::consts::PI, 0.0, 1.0);
        let updown = util::map(stack, -std::f32::consts::PI/2.0, std::f32::consts::PI/2.0, 0.0, 1.0);
        
        selected.x = f32::floor(around * 8.0) as i8;
        selected.y = f32::floor(updown * 8.0) as i8;

        let around = util::map(selected.x as f32, 0.0, 8.0, -std::f32::consts::PI, std::f32::consts::PI);
        let updown = util::map(selected.y as f32, 0.0, 8.0, -std::f32::consts::PI/2.0, std::f32::consts::PI/2.0,);

        let position_ss = stacks_and_sectors_to_sphere_position(updown, around, sphere.radius);
        giz_transform.translation = position_ss;

        // println!("{},{}", selected.x, selected.y);
    }
}

