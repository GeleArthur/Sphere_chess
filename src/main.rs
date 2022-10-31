use bevy::{
    input::mouse::MouseMotion,
    math::*,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WireframePlugin)
        .add_startup_system(setup_scene)
        .add_system(camera_rotation)
        .add_system(camera_sphere_select)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        })
        .insert(Name::new("Camera"))
        .insert(CameraRotation { x: 0.0, y: 0.0 });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Wireframe);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.1).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Plane"));

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 1.0),
        ..Default::default()
    });
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct CameraRotation {
    x: f32,
    y: f32,
}

fn camera_rotation(
    mut camera_query: Query<(&mut Transform, &mut CameraRotation), With<Camera3d>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.pressed(MouseButton::Left) == false {
        return;
    }
    if motion_evr.len() == 0 {
        return;
    }

    let one_query = camera_query.single_mut();
    let mut camera = one_query.0;
    let mut camera_rot = one_query.1;

    for ev in motion_evr.iter() {
        camera_rot.x -= ev.delta.x * 0.005;
        camera_rot.y -= ev.delta.y * 0.005;

        camera.rotation = Quat::from_axis_angle(Vec3::Y, camera_rot.x)
            * Quat::from_axis_angle(Vec3::X, camera_rot.y);

        let forward = camera.forward();
        camera.translation = -forward * 5.0;
    }
}

fn camera_sphere_select(query: Query<(&Camera, &GlobalTransform)>, windows: Res<Windows>) {

    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        let camera_pos = query.single();

        let pos = from_screenspace(_position, camera_pos.0, camera_pos.1).unwrap();

        print!("{} {} {} \n", pos.0.x, pos.0.y, pos.0.z);
    }
}

pub fn from_screenspace(
    cursor_pos_screen: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<(Vec3, Vec3)> {
    let view = camera_transform.compute_matrix();

    let (viewport_min, viewport_max) = camera.logical_viewport_rect()?;
    let screen_size = camera.logical_target_size()?;
    let viewport_size = viewport_max - viewport_min;
    let adj_cursor_pos =
        cursor_pos_screen - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);

    let projection = camera.projection_matrix();
    let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
    let near_ndc = projection.project_point3(Vec3::Z).z;
    let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
    let ndc_to_world: Mat4 = view * projection.inverse();
    let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
    let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
    let ray_direction = far - near;

    return Some((near, ray_direction));
}
