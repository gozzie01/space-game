// if you name the crate SpaceEngine it for some reason runs at half speed, blame the rust compiler idek
mod input;
mod physics;
mod render;
mod camera;
use std::vec;
use input::*;
use camera::*;
use physics::*;
use render::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::prelude::*;
use ultraviolet::DVec2;

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
            _name: "Sun".to_string(),
            position: DVec2::new(0.0, 0.0),
            velocity: DVec2::new(0.0, 0.0),
            radius: 4.0,
            mass: 2.0e30, // Solar mass
        },
        Body {
            _name: "Sun2".to_string(),
            position: DVec2::new(4e11, 0.0),
            velocity: DVec2::new(0.0, 1e4),
            radius: 4.0,
            mass: 2.0e30, // Solar mass
        },
        Body {
            _name: "Earth".to_string(),
            position: DVec2::new(1.496e11, 0.0),   // 1 AU
            velocity: DVec2::new(0.0, 30000.0),
            radius: 2.0, // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
        Body {
            _name: "Venus".to_string(),
            position: DVec2::new(1.08e11, 0.0),   // venus
            velocity: DVec2::new(0.0, 35000.0),
            radius: 2.0, // km/s scaled down
            mass: 4.868e24,                        // Venus mass
        },
        Body {
            _name: "Mars".to_string(),
            position: DVec2::new(2.28e11, 0.0),   // mars
            velocity: DVec2::new(0.0, 24000.0),
            radius: 2.0, // km/s scaled down
            mass: 6.42e23,                        // mars mass
        },
    ]
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
    ))
    .add_systems(Startup, setup);
    app.add_systems(Update, update_bodies_system);
    app.add_systems(Update, render_bodies_system);
    app.add_systems(Update, calculate_center_of_mass_system);
    app.add_systems(Update, update_camera_system);
    app.add_systems(Update, mouse_system);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    intialize_camera(&mut commands);
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
    let mut bodies: Vec<(Position, Velocity, Mass)> = query.iter_mut().map(|(pos, vel, mass)| {
        (Position(pos.0), Velocity(vel.0), Mass(mass.0))
    }).collect();

    let forces = compute_forces_tuple(&bodies);
    update_bodies_tuple(&mut bodies, forces, dt * 1000000.0);

    for ((mut pos, mut vel, _), body) in query.iter_mut().zip(bodies.iter()) {
        pos.0 = body.0.0;
        vel.0 = body.1.0;
    }
}


