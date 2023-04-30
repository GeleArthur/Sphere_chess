use std::f32::consts::PI;

use crate::*;
use bevy::prelude::*;

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
            .add_startup_system(spawn_camera)
            .add_system(camera_rotation)
            .add_system(light_to_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        })
        .insert(Name::new("Camera"))
        .insert(CameraRotation { x: 0.0, y: 0.0 });
}

fn camera_rotation(
    mut camera_query: Query<(&mut Transform, &mut CameraRotation), With<Camera3d>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut reverse_motion: Local<bool>
) {
    let (mut camera, mut camera_rot) = camera_query.single_mut();

    if buttons.just_pressed(MouseButton::Left) {
        if camera_rot.y < PI+PI/2.0 && camera_rot.y > PI/2.0 {
            *reverse_motion = true;
        }else{
            *reverse_motion = false;
        }
    }

    if motion_evr.len() == 0 {
        return;
    }
    if buttons.pressed(MouseButton::Left) == false {
        return;
    }


    for ev in motion_evr.iter() {

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
    light.single_mut().translation = camera.single().translation;
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