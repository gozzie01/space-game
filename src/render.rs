use bevy::prelude::*;
use crate::Position;

pub fn render_bodies_system(
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new((pos.0.x / 1e9) as f32, (pos.0.y / 1e9) as f32, 0.0);
    }
}