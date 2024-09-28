// if you name the crate SpaceEngine it for some reason runs at half speed, blame the rust compiler idek
mod input;
mod physics;
mod render;
mod camera;
use std::vec;
use bevy::window::PresentMode;
use input::*;
use camera::*;
use iyes_perf_ui::prelude::*;
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

#[derive(Resource)]
struct SpeedScalar(f64);

#[derive(Resource)]
struct PrecisionScalar(u32);

fn initialize_bodies() -> Vec<Body> {
    vec![
        Body {
            _name: "Sun".to_string(),
            position: DVec2::new(0.0, 0.0),
            velocity: DVec2::new(0.0, 0.0),
            radius: 4.0,
            mass: 2.0e30,                          // Solar mass
        },
        Body {
            _name: "Earth".to_string(),
            position: DVec2::new(1.496e11, 0.0),   // 1 AU
            velocity: DVec2::new(0.0, 29780.0),    // km/s scaled down
            radius: 1.0, 
            mass: 5.972e24,                        // Earth mass
        },
        Body {
            _name: "Moon".to_string(),
            position: DVec2::new(1.49985e11, 0.0),   // 1 AU
            velocity: DVec2::new(0.0, 30802.0),    // km/s scaled down
            radius: 0.3, 
            mass: 7.35e22,                        // Earth mass
        },
        Body {
            _name: "Venus".to_string(),
            position: DVec2::new(1.08e11, 0.0),   // venus
            velocity: DVec2::new(0.0, 35000.0),   // km/s scaled down
            radius: 1.0, 
            mass: 4.868e24,                        // Venus mass
        },
        Body {
            _name: "Mars".to_string(),
            position: DVec2::new(2.2794e11, 0.0),   // mars
            velocity: DVec2::new(0.0, 24000.0),   // km/s scaled down
            radius: 1.0, 
            mass: 6.42e23,                        // mars mass
        },
        Body {
            _name: "Mercury".to_string(),
            position: DVec2::new(4.7e10, 0.0),   // mars
            velocity: DVec2::new(0.0, 47870.0),   // km/s scaled down
            radius: 1.0, 
            mass: 3.302e23,                        // mars mass
        },
        Body {
            _name: "Jupiter".to_string(),
            position: DVec2::new(7.78e11, 0.0),   // mars
            velocity: DVec2::new(0.0, 13070.0),   // km/s scaled down
            radius: 3.0, 
            mass: 1.8987e27,                        // mars mass
        },
        Body {
            _name: "Saturn".to_string(),
            position: DVec2::new(1.434e12, 0.0),   // mars
            velocity: DVec2::new(0.0, 9680.0),   // km/s scaled down
            radius: 3.0, 
            mass: 5.6851e26,                        // mars mass
        },
    ]
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }),
    ))
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)

    .add_plugins(PerfUiPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, update_bodies_system)
    .add_systems(Update, render_bodies_system)
    .add_systems(Update, calculate_center_of_mass_system)
    //.add_systems(Update, update_camera_system)
    .add_systems(Update, mouse_system)
    .add_systems(Update, scroll_system)
    .add_systems(Update, modify_speed_scalar_system)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    intialize_camera(&mut commands);
    commands.insert_resource(CenterOfMass::default());
    commands.insert_resource(SpeedScalar(1e6)); // Initialize with a default value
    commands.insert_resource(PrecisionScalar(10)); // Initialize with a default value

    let bodies = initialize_bodies();
    
    for body in bodies {
        let radius = body.radius;
        commands.spawn((
            PerfUiRoot::default(),
            PerfUiEntryFPS::default(),
            PerfUiEntryClock::default(),
            // ...
         ));
        commands.spawn((
            Position(body.position),
            Velocity(body.velocity),
            Mass(body.mass),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle { radius }).into(),
                material: materials.add(Color::srgb(2.0 * radius * radius, 0.0, 7.5)),
                transform: Transform::from_translation(Vec3::new(body.position.x as f32, body.position.y as f32, 0.0)),
                ..default()}
        ));
    }
}

fn update_bodies_system(
    mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
    time: Res<Time>,
    speed_scalar: Res<SpeedScalar>,
    precision_scalar: Res<PrecisionScalar>,
) {
    let dt = time.delta_seconds_f64() * speed_scalar.0;
    let mut bodies: Vec<(Position, Velocity, Mass)> = query.iter_mut().map(|(pos, vel, mass)| {
        (Position(pos.0), Velocity(vel.0), Mass(mass.0))
    }).collect();

    physics_sim(&mut bodies, dt, precision_scalar.0);

    for ((mut pos, mut vel, _), body) in query.iter_mut().zip(bodies.iter()) {
        pos.0 = body.0.0;
        vel.0 = body.1.0;
    }
}


