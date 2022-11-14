use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseMotion,
    math::*,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct CameraRotation {
    x: f32,
    y: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct GizmosCube;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct CenterSphere {
    center: Vec3,
    radius: f32,
}

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
                // sectors:8,
                // stacks: 3,
                ..Default::default()
            })),
            material: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.1).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                PI / 2.0,
                0.0,
                0.0,
            )),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert(Name::new("Sphere"));

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
        // print!("{} {}\n", stack_sector.0, stack_sector.1);
        let position_ss = stacks_and_sectors_to_sphere_position(stack_sector.0, stack_sector.1, sphere.radius);
        // print!("{}\n", position_ss);



        giz_transform.translation = position_ss;

        //giz_transform.translation = to_sphere - Vec3::project_onto(to_sphere, raycasted.1);

        //let v = to_sphere - Vec3::project_onto(to_sphere, raycasted.1);
        // let v_squerd = v.dot(v);
        // let r_squard = sphere.radius * sphere.radius;

        // if v_squerd <= r_squard {
        //     let collison_point =
        //         sphere.center + v - (raycasted.1.normalize() * (f32::sqrt(r_squard - v_squerd)));

        //     giz_transform.translation = collison_point;

        //     let pitch = f32::atan2(collison_point.x, collison_point.z);
        //     let yaw = collison_point.y;

        //     let xy = yaw.cos();
        //     let x = xy * pitch.cos();
        //     let y = xy * pitch.sin();

        //     let converted_pos = Vec3::new(x, yaw, y);

        //     giz_transform.translation = converted_pos;

        //     print!("pitch:{} yaw:{}\n", pitch, yaw);
        // }
    }
}

fn raycast_ball(ray_cast: (Vec3, Vec3), sphere_position: Vec3, sphere_radius: f32) -> Option<Vec3> {
    let to_sphere = ray_cast.0 - sphere_position;

    let v = to_sphere - Vec3::project_onto(to_sphere, ray_cast.1.normalize());
    let v_squard = v.dot(v);
    let r_squard = sphere_radius * sphere_radius;

    if v_squard <= r_squard {
        return Some(
            sphere_position + v - (ray_cast.1.normalize() * (f32::sqrt(r_squard - v_squard))),
        );
    } else {
        return None;
    }
}

// thanks tibor and ruben :D
fn sphere_position_to_stacks_and_sectors(position: Vec3, radius: f32) -> (f32,f32){
    let stack_angle = (position.z/radius).asin();


    let sector_angle = (position.x/(radius*stack_angle.cos())).acos();
    let sector_angle2 = (position.y/(radius*stack_angle.cos())).asin();

    print!("{sector_angle}, {sector_angle2}\n");

    return (stack_angle, sector_angle);
}


fn stacks_and_sectors_to_sphere_position(stack: f32 /*pi/2 to -pi/2*/, sector: f32 /*0 to 2pi*/, radius: f32) -> Vec3{
    let xy = radius * stack.cos();
    let x = xy * sector.cos();
    let y = xy * sector.sin();

    let z = radius * stack.sin();

    return Vec3::new(x, y, z);
}

fn from_screenspace(
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
