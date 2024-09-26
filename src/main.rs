// if you name the crate SpaceEngine it for some reason runs at half speed, blame the rust compiler idek
use pixels::wgpu::PresentMode;
use pixels::{Error, Pixels, SurfaceTexture};
use ultraviolet::DVec3;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

struct Body {
    position: DVec3,
    velocity: DVec3,
    mass: f64,
}

fn initialize_bodies() -> Vec<Body> {
    vec![
        Body {
            position: DVec3::new(0.0, 0.0, 0.0),
            velocity: DVec3::new(0.0, 0.0, 0.0),
            mass: 1.0e30, // Solar mass
        },
        Body {
            position: DVec3::new(1.1, 0.0, 0.0),
            velocity: DVec3::new(0.0, 0.000000000000098, 0.0),
            mass: 1.0e30, // Solar mass
        },
        Body {
            position: DVec3::new(1.0, 0.0, 0.0),   // 1 AU
            velocity: DVec3::new(0.0, 0.000000000198, 0.0), // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
        Body {
            position: DVec3::new(-1.0, 0.0, 0.0),   // 1 AU
            velocity: DVec3::new(0.0, -0.000000000198, 0.0), // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
    ]
}

fn compute_forces(bodies: &Vec<Body>) -> Vec<DVec3> {
    let mut forces = vec![DVec3::zero(); bodies.len()];
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let direction = bodies[j].position - bodies[i].position;
            let distance = direction.mag()  + 1e-10; // Avoid division by zero
            let force_magnitude = ((bodies[i].mass * bodies[j].mass) / (distance * distance)).min(1e5); // Clamp force
            let force: DVec3 = direction.normalized() * force_magnitude;
            forces[i] += force;
            forces[j] -= force; // Newton's third law: equal and opposite force
        }
    }
    forces
}

fn update_bodies(bodies: &mut Vec<Body>, forces: Vec<DVec3>, dt: f64) {
    for (body, force) in bodies.iter_mut().zip(forces.iter()) {
        let acceleration = *force / body.mass;
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
    for _ in 0..100000 {
        let forces = compute_forces(&bodies);
        update_bodies(&mut bodies, forces, dt);
        // Optionally, print or visualize the positions of the bodies

        //for body in bodies.iter() {
        //    println!("{:?}", body.position);
        //}
    }
    let duration = start.elapsed();
    println!("Time elapsed in computation is: {:?}", duration);
}*/

fn main() -> Result<(), Error> {
    // Create an event loop
    let event_loop = EventLoop::new().unwrap();
    let mut bodies = initialize_bodies();
    let dt = 6900000.0; // Time step

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Space-Engine")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.set_present_mode(PresentMode::Immediate);

    let res = event_loop.run(|event, elwt| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event

        {
            let frame = pixels.frame_mut();
            for p in frame.chunks_mut(4) {
                p[0] = 0;
                p[1] = 0;
                p[2] = 0;
                p[3] = 255;
            }
            //set the closest pixel to each body to white, 0,0 is centre 1.5 is far right -1.5 is far left, 1.5 is top, -1.5 is bottom
            for body in bodies.iter() {
                let x = (body.position.x + 1.5) * 100.0;
                let y = (body.position.y + 1.5) * 100.0;
                let x = x as usize;
                let y = y as usize;
                let i = (x + y * WIDTH as usize) * 4;
                if i < frame.len() {
                    frame[i] = 255;
                    frame[i + 1] = 255;
                    frame[i + 2] = 255;
                    frame[i + 3] = 255;
                }
            }
            if let Err(err) = pixels.render() {
                println!("pixels.render, {0}", err.to_string());
                elwt.exit();
                return;
            }
        }
        if let Event::AboutToWait = event {
            let forces = compute_forces(&bodies);
            update_bodies(&mut bodies, forces, dt);
            // Request a redraw
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}
