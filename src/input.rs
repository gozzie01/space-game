
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::Window;
use ultraviolet::DVec2;

use crate::Position;
use crate::Velocity;
use crate::Mass;

pub fn mouse_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        println!("Mouse: {:?}", mouse_button_input.get_pressed().collect::<Vec<_>>());
        let d_world_position = DVec2::new(world_position.x as f64, world_position.y as f64) * 1e9;
        let radius = 2.0;
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                Position(d_world_position),
                Velocity(DVec2::zero()),
                Mass(5.972e24),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle { radius }).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(d_world_position.x as f32, d_world_position.y as f32, 0.0)),
                    ..default()
                },
            ));
        }
    }
}