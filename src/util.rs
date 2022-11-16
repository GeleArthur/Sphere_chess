use bevy::prelude::*;

pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    return  (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
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

pub fn raycast_ball(ray_cast: (Vec3, Vec3), sphere_position: Vec3, sphere_radius: f32) -> Option<Vec3> {
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

pub fn sphere_position_to_stacks_and_sectors(position: Vec3, _radius: f32) -> (f32,f32){
    let phi = f32::atan2(position.z, position.x);
    let theta = f32::asin(position.y);

    return (theta, phi);
}


pub fn stacks_and_sectors_to_sphere_position(stack: f32 /*pi/2 to -pi/2*/, sector: f32 /*0 to 2pi*/, radius: f32) -> Vec3{
    let xz = radius * stack.cos();
    let x = xz * sector.cos();
    let z = xz * sector.sin();

    let y = radius * stack.sin();

    return Vec3::new(x, y, z);
}