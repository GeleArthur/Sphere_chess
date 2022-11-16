use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;

use crate::*;
use crate::util::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct CenterSphere {
    center: Vec3,
    radius: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChessBoard {
    pub grid: [[f32; 8]; 8],
}

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_board)
        .add_system(camera_sphere_select)
        ;
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 1.0;

    commands
        .spawn()
        .insert(CenterSphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: radius,
        })
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: radius,
                sectors: 8,
                stacks: 8,
                ..Default::default()
            })),
            material: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.1).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                std::f32::consts::PI / 2.0,
                0.0,
                0.0,
            )),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert(Name::new("Sphere"));
}

fn camera_sphere_select(
    camera: Query<(&Camera, &GlobalTransform)>,
    mut gizmos_cube: Query<&mut Transform, With<GizmosCube>>,
    sphere: Query<&CenterSphere>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_position) = window.cursor_position() {
        let camera_pos = camera.single();

        let raycasted = from_screenspace(mouse_position, camera_pos.0, camera_pos.1).unwrap();
        let mut giz_transform = gizmos_cube.single_mut();

        let sphere = sphere.single();

        let ray_hit = raycast_ball(raycasted, sphere.center, sphere.radius);

        if ray_hit.is_none() {
            return;
        }

        let hit = ray_hit.unwrap();

        let stack_sector = sphere_position_to_stacks_and_sectors(hit, sphere.radius);

        let sector = stack_sector.1;
        let stack = stack_sector.0;

        // print!("{stack}, {sector}\n");

        let between = util::map(stack, -std::f32::consts::PI/2.0, std::f32::consts::PI/2.0, 0.0, 1.0);
        let around = util::map(sector, -std::f32::consts::PI, std::f32::consts::PI, 0.0, 1.0);

        println!("{around}");

        let position_ss = stacks_and_sectors_to_sphere_position(stack, sector, sphere.radius);

        giz_transform.translation = position_ss;
    }
}