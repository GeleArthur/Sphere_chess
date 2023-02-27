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
) {
    if motion_evr.len() == 0 {
        return;
    }
    if buttons.pressed(MouseButton::Left) == false {
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

fn light_to_camera(
    camera: Query<&Transform, (With<Camera>, Without<PointLight>)>,
    mut light: Query<&mut Transform, (With<PointLight>, Without<Camera>)>,
) {
    light.single_mut().translation = camera.single().translation;
}
