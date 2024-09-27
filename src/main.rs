use std::vec;
use bevy::input::mouse::MouseButtonInput;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::transform::commands;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::window::Window;

// if you name the crate SpaceEngine it for some reason runs at half speed, blame the rust compiler idek
use ultraviolet::DVec2;

#[derive(Clone)]
struct Body {
    name: String,
    position: DVec2,
    velocity: DVec2,
    radius: f32,
    mass: f64,
}

#[derive(Component)]
struct Position(DVec2);

#[derive(Component)]
struct Velocity(DVec2);

#[derive(Component)]
struct Mass(f64);

#[derive(Component, Resource)]
struct CenterOfMass(DVec2);

impl Default for CenterOfMass {
    fn default() -> Self {
        CenterOfMass(DVec2::zero())
    }
}


fn initialize_bodies() -> Vec<Body> {
    vec![
        Body {
            name: "Sun".to_string(),
            position: DVec2::new(0.0, 0.0),
            velocity: DVec2::new(0.0, 0.0),
            radius: 4.0,
            mass: 2.0e30, // Solar mass
        },
        Body {
            name: "Sun2".to_string(),
            position: DVec2::new(4e11, 0.0),
            velocity: DVec2::new(0.0, 1e4),
            radius: 4.0,
            mass: 2.0e30, // Solar mass
        },
        Body {
            name: "Earth".to_string(),
            position: DVec2::new(1.496e11, 0.0),   // 1 AU
            velocity: DVec2::new(0.0, 30000.0),
            radius: 2.0, // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
        Body {
            name: "Venus".to_string(),
            position: DVec2::new(1.08e11, 0.0),   // venus
            velocity: DVec2::new(0.0, 35000.0),
            radius: 2.0, // km/s scaled down
            mass: 4.868e24,                        // Venus mass
        },
        Body {
            name: "Mars".to_string(),
            position: DVec2::new(2.28e11, 0.0),   // mars
            velocity: DVec2::new(0.0, 24000.0),
            radius: 2.0, // km/s scaled down
            mass: 6.42e23,                        // mars mass
        },
    ]
}

fn compute_forces(bodies: &Vec<Body>) -> Vec<DVec2> {
    let mut forces = vec![DVec2::zero(); bodies.len()];
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let direction = bodies[j].position - bodies[i].position;
            let distance = direction.mag()   + 100.0; // Avoid division by zero
            let force_magnitude = (6.6743e-11 * (bodies[i].mass * bodies[j].mass)) / (distance * distance); // Clamp force
            let force: DVec2 = direction.normalized() * force_magnitude ;
            forces[i] += force;
            forces[j] -= force; // Newton's third law: equal and opposite force
        }
    }
    forces
}

fn update_bodies(bodies: &mut Vec<Body>, forces: Vec<DVec2>, dt: f64) {
    for (body, force) in bodies.iter_mut().zip(forces.iter()) {
        let acceleration = *force / body.mass  ;
        body.velocity += acceleration * dt;
        body.position += body.velocity * dt;
        
    }
}
/* 
fn main() {
    let start = std::time::Instant::now();
    let mut bodies = initialize_bodies();
    let dt = 69000000.0; // Time step
    for body in bodies.iter() {
        println!("{:?}", body.position);
    }
    for _ in 0..10 {
        let forces = compute_forces(&bodies);
        update_bodies(&mut bodies, forces, dt);
        // Optionally, print or visualize the positions of the bodies

        for body in bodies.iter() {
            println!("{:?}", body.position);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in computation is: {:?}", duration);
}
*/

fn main() {
    let mut bodies = initialize_bodies();
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        Wireframe2dPlugin,
    ))
    .add_systems(Startup, setup);
    app.add_systems(Update, update_bodies_system);
    app.add_systems(Update, render_bodies_system);
    app.add_systems(Update, calculate_center_of_mass_system);
    app.add_systems(Update, update_camera_system);
    app.add_systems(Update, mouse_system);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(CenterOfMass::default());

    let bodies = initialize_bodies();
    for body in bodies {
        let radius = body.radius;
        commands.spawn((
            Position(body.position),
            Velocity(body.velocity),
            Mass(body.mass),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle { radius }).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(body.position.x as f32, body.position.y as f32, 0.0)),
                ..default()
            },
        ));
    }
}

fn update_bodies_system(
    mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds_f64();
    println!("dt: {}", dt);
    let mut bodies: Vec<Body> = query.iter_mut().map(|(pos, vel, mass)| Body {
        name: "".to_string(),
        position: pos.0,
        velocity: vel.0,
        radius: 2.0,
        mass: mass.0,
    }).collect();

    let forces = compute_forces(&bodies);
    update_bodies(&mut bodies, forces, dt * 1000000.0);

    for ((mut pos, mut vel, _), body) in query.iter_mut().zip(bodies.iter()) {
        pos.0 = body.position;
        vel.0 = body.velocity;
    }
}

fn render_bodies_system(
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new((pos.0.x / 1e9) as f32, (pos.0.y / 1e9) as f32, 0.0);
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn calculate_center_of_mass_system(
    query: Query<(&Position, &Mass)>,
    mut center_of_mass: ResMut<CenterOfMass>,
) {
    let mut total_mass = 0.0;
    let mut weighted_position_sum = DVec2::zero();

    for (pos, mass) in query.iter() {
        total_mass += mass.0;
        weighted_position_sum += pos.0 * mass.0;
    }

    if total_mass > 0.0 {
        center_of_mass.0 = weighted_position_sum / total_mass;
    }
}

fn update_camera_system(
    center_of_mass: Res<CenterOfMass>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new((center_of_mass.0.x / 1e9) as f32, (center_of_mass.0.y / 1e9) as f32, transform.translation.z);
    }
}
fn mouse_system(
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
                Position((d_world_position)),
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