
use bevy::prelude::*;
use crate::{Position, Radius};

pub fn render_bodies_system(
    mut query: Query<(&Position, &Radius, &mut Transform)>,
    mut query_camera: Query<&mut OrthographicProjection, With<Camera>>
) {
    for (pos, _radius, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new((pos.0.x / 1e9) as f32, (pos.0.y / 1e9) as f32, 0.0);
    }
    //scale the bodies to make them visible from zoomed out but as the camera zooms in the bodies will be scaled down to be closer to the actual size in the default view the sun should be roughly 6 pixels in radius and the earth should be roughly 2 pixel in radius
    let current_zoom = query_camera.iter_mut().next().unwrap().scale;
    println!("current zoom: {}", current_zoom);
    for (_pos, radius, mut transform) in query.iter_mut() {
        let scale = calculate_scale(radius.0, current_zoom);
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
fn calculate_scale(real_radius: f32, zoom: f32) -> f32 {
    // Assume some real radii for these celestial bodies
    let scale_1 = f32::powi(real_radius.log10()/6., 6)/real_radius;
    let scale_100 = 1./1e8_f32;
    //interpolate between the two scales based on the zoom level, zoom level should be scale 100 at 0.1 and scale 1 at 1.0
    if (zoom < 0.05) {
        return scale_100;
    } else if (zoom > 1.0) {
        return scale_1;
    } else {
        return scale_100 + (scale_1 - scale_100) * (zoom - 0.05) / 1.95;
    }
    
}