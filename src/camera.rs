use crate::CenterOfMass;
use bevy::prelude::*;

pub fn update_camera_system(
    center_of_mass: Res<CenterOfMass>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new((center_of_mass.0.x / 1e9) as f32, (center_of_mass.0.y / 1e9) as f32, transform.translation.z);
    }
}