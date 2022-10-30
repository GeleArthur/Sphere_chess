use bevy::{
    input::mouse::MouseMotion,
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
