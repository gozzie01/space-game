use std::vec;

// if you name the crate SpaceEngine it for some reason runs at half speed, blame the rust compiler idek
use pixels::wgpu::PresentMode;
use pixels::{Error, Pixels, SurfaceTexture};
use ultraviolet::Vec3;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

#[derive(Clone)]
struct Body {
    position: Vec3,
    velocity: Vec3,
    mass: f32,
}

fn initialize_bodies() -> Vec<Body> {
    vec![
        Body {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            mass: 1.0e30, // Solar mass
        },
        Body {
            position: Vec3::new(1.1, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.000000000000098, 0.0),
            mass: 1.0e30, // Solar mass
        },
        Body {
            position: Vec3::new(1.0, 0.0, 0.0),   // 1 AU
            velocity: Vec3::new(0.0, 0.000000000198, 0.0), // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
        Body {
            position: Vec3::new(-1.0, 0.0, 0.0),   // 1 AU
            velocity: Vec3::new(0.0, -0.000000000198, 0.0), // km/s scaled down
            mass: 5.972e24,                        // Earth mass
        },
    ]
}

fn compute_forces(bodies: &Vec<Body>) -> Vec<Vec3> {
    let mut forces = vec![Vec3::zero(); bodies.len()];
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let direction = bodies[j].position - bodies[i].position;
            let distance = direction.mag()  + 1e-10; // Avoid division by zero
            let force_magnitude = ((bodies[i].mass * bodies[j].mass) / (distance * distance)).min(1e5); // Clamp force
            let force: Vec3 = direction.normalized() * force_magnitude;
            forces[i] += force;
            forces[j] -= force; // Newton's third law: equal and opposite force
        }
    }
    forces
}

fn update_bodies(bodies: &mut Vec<Body>, forces: Vec<Vec3>, dt: f32) {
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

    // Create input handler
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new().unwrap();
    let bodies = Arc::new(Mutex::new(initialize_bodies()));
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

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let bodies_clone = Arc::clone(&bodies);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    thread::spawn(move || {
        while running_clone.load(Ordering::SeqCst) {
            // Wait for the signal from the main thread
            if rx.recv().is_err() {
                break;
            }

            let mut bodies = bodies_clone.lock().unwrap();
            let forces = compute_forces(&bodies);
            update_bodies(&mut bodies, forces, dt);
        }
    });

    let res = event_loop.run(|event, elwt| {
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            running.store(false, Ordering::SeqCst);
            tx.send(()).unwrap(); // Unblock the worker thread if it's waiting
            elwt.exit();
        }

        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let frame = pixels.frame_mut();
            //set it to black
            frame.fill(0);
            let bodies = bodies.lock().unwrap();
            let render_bodies = bodies.clone();
            tx.send(()).unwrap();
            //set the closest pixel to each body to white, 0,0 is centre 1.5 is far right -1.5 is far left, 1.5 is top, -1.5 is bottom
            for body in render_bodies.iter() {
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
            // Request a redraw
            window.request_redraw();
        }

        //Input Handling
        if input.update(&event) {
            if input.mouse_pressed(0) {
                if let Some((mx, my)) = input.cursor() {
                    //println!("Mouse clicked at: {:?}", Vec3::new((mx / 100.0) -1.5, (my / 100.0) -1.5, 0.0));
                    let mouse_win_coords = pixels.window_pos_to_pixel((mx, my)).unwrap();
                    println!("Mouse clicked at: {:?}", Vec3::new(mouse_win_coords.0 as f32 / 100.0 - 1.5, mouse_win_coords.1 as f32 / 100.0 - 1.5, 0.0));
                    let mut bodies = bodies.lock().unwrap();
                    add_body(&mut bodies, Vec3::new(mouse_win_coords.0 as f32 / 100.0 - 1.5, mouse_win_coords.1 as f32 / 100.0 - 1.5, 0.0));
                }
            }
        }

    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn add_body(bodies: &mut Vec<Body>, mouse_coords: Vec3) {
    bodies.append(&mut vec![Body {
        position: mouse_coords,
        velocity: Vec3::new(0.0, 0.0, 0.0), // km/s scaled down
        mass: 5.972e24, // Earth mass
    }]);
}
