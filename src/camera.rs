use std::f32::consts::PI;

use crate::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CameraRotation {
    pub x: f32,
    pub y: f32,
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraRotation>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_rotation)
            .add_systems(Update,light_to_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 5.0), Name::new("Camera"), CameraRotation{x:0.0, y:0.0}));


            // transform: Transform::from_xyz(0.0, 0.0, 5.0),
            // ..Default::default()
        // })
        // .insert(Name::new("Camera"))
        // .insert(CameraRotation { x: 0.0, y: 0.0 });
}

fn camera_rotation(
    camera_query: Single<(&mut Transform, &mut CameraRotation), With<Camera3d>>,
    mut mouse_move: MessageReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut reverse_motion: Local<bool>
) {
    let (mut camera, mut camera_rot) = camera_query.into_inner();

    if buttons.just_pressed(MouseButton::Left) {
        if camera_rot.y < PI+PI/2.0 && camera_rot.y > PI/2.0 {
            *reverse_motion = true;
        }else{
            *reverse_motion = false;
        }
    }

    if buttons.pressed(MouseButton::Left) == false {
        return;
    }


    for ev in mouse_move.read() {

        if *reverse_motion {
            camera_rot.x += ev.delta.x * 0.005;
        }else{
            camera_rot.x -= ev.delta.x * 0.005;
        }

        camera_rot.y -= ev.delta.y * 0.005;

        camera_rot.x = normalize_rotation(camera_rot.x);
        camera_rot.y = normalize_rotation(camera_rot.y);


        camera.rotation = Quat::from_axis_angle(Vec3::Y, camera_rot.x) * Quat::from_axis_angle(Vec3::X, camera_rot.y);

        let forward = camera.forward();
        camera.translation = -forward * 5.0;
    }
}

fn light_to_camera(
    camera: Query<&Transform, (With<Camera>, Without<PointLight>)>,
    mut light: Query<&mut Transform, (With<PointLight>, Without<Camera>)>,
) {
    light.single_mut().unwrap().translation = camera.single().unwrap().translation;
}

fn normalize_rotation(mut rotation: f32) -> f32{
    if rotation > PI*2.0 {
        rotation -= PI*2.0;
        return normalize_rotation(rotation);
    }else if rotation < 0.0 {
        rotation += PI*2.0;
        return normalize_rotation(rotation);
    }

    rotation
}