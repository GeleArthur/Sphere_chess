mod util;
mod camera;
mod chess;

use camera::*;
use chess::*;

use bevy::{
    input::mouse::MouseMotion,
    math::*,
    pbr::wireframe::{WireframePlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct GizmosCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ChessPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.1).into()),
            ..Default::default()
        })
        .insert(GizmosCube)
        .insert(Name::new("Gizmos"));
}
