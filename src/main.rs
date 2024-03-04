
// https://www.roxlu.com/2014/037/opengl-rim-shader
// https://www.youtube.com/watch?v=pWYt348Ki5g
mod camera;
mod chess;
mod constant;
mod game_assets;
mod pieces;
mod util;
mod sphere_material;

use camera::*;
use chess::*;
use sphere_material::ChessSphereMaterial;
use pieces::*;

use bevy::{
    input::mouse::MouseMotion,
    math::*,
    prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct GizmosCube;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Sphere chess".to_owned(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            }).set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
        )
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, game_assets::asset_loading)
        .add_plugin(CameraPlugin)
        .add_plugin(ChessPlugin)
        .add_plugin(PiecePlugin)
        .add_plugin(MaterialPlugin::<ChessSphereMaterial>::default())
        .add_plugin(MaterialPlugin::<PiecesMaterial>::default())
        .add_event::<PieceClickedEvent>()
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 1.0),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                Mesh::from(shape::UVSphere {
                    radius: 0.1,
                    ..Default::default()
                })
            ),
            material: materials.add(Color::rgb(1.0, 0.0, 0.1).into()),
            ..Default::default()
        })
        .insert(GizmosCube)
        .insert(Name::new("Gizmos"));
}