use crate::camera::*;

use bevy::ecs::query;
use bevy::prelude::*;
use bevy::render::camera;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::Window;
use bevy::input::mouse::MouseWheel;
use ultraviolet::DVec2;

use crate::Position;
use crate::Velocity;
use crate::Mass;

#[derive(Component)]
pub struct MyCameraMarker;

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
        let d_world_position = DVec2::new(world_position.x as f64, world_position.y as f64) * 1e9;
        let radius = 2.0;
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                Position(d_world_position),
                Velocity(DVec2::zero()),
                Mass(5.972e24),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle { radius }).into(),
                    material: materials.add(Color::srgb(2.0 * radius, 0.0, 7.5)),
                    transform: Transform::from_translation(Vec3::new(d_world_position.x as f32, d_world_position.y as f32, 0.0)),
                    ..default()
                },
            ));
        }
    }
}

pub fn scroll_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query_camera: Query<&mut OrthographicProjection, With<Camera>>
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                match query_camera.get_single_mut() {
                    Ok(mut projection) => {
                        projection.scale *= ev.y * 0.1;
                    }
                    Err(e) => {
                        eprintln!("Failed to get camera projection: {:?}", e);
                    }
                }
            }
            MouseScrollUnit::Pixel => {

            }
        }
    }
}

pub fn modify_speed_scalar_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut speed_scalar: ResMut<crate::SpeedScalar>,
    mut precision_scalar: ResMut<crate::PrecisionScalar>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        speed_scalar.0 *= 1.1; // Increase speed
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        speed_scalar.0 *= 0.9; // Decrease speed
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        precision_scalar.0 *= 2; // Increase precision
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        precision_scalar.0 /= 2; // Decrease precision
    }
}