
// https://www.roxlu.com/2014/037/opengl-rim-shader
// https://www.youtube.com/watch?v=pWYt348Ki5g
mod camera;
mod chess;
mod game_assets;
mod pieces;
mod util;
mod sphere_material;

use camera::*;
use chess::*;
use sphere_material::ChessSphereMaterial;
use pieces::*;

use bevy::{
    prelude::*, window::PresentMode,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct GizmosCube;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sphere chess".to_owned(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }).set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            })
        )
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, game_assets::asset_loading)
        .add_plugins(CameraPlugin)
        .add_plugins(ChessPlugin)
        .add_plugins(PiecePlugin)
        .add_plugins(MaterialPlugin::<ChessSphereMaterial>::default())
        .add_plugins(MaterialPlugin::<PiecesMaterial>::default())
        .add_message::<PieceClickedEvent>()
        .add_systems(Startup,setup_scene.after(game_assets::asset_loading))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
         PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 2.0, 1.0)
    ));
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(0.1)))), 
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 10))), 
        Transform::from_xyz(0.0, 0.0, 0.0) 
    ));

    commands
        .spawn(( 
            Mesh3d(meshes.add(
                Mesh::from(Sphere::new(0.1))
            )),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 10))),
        ))
        .insert(GizmosCube)
        .insert(Name::new("Gizmos"));
}