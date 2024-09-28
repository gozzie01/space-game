use bevy::prelude::*;
use ultraviolet::DVec2;

use crate::{CenterOfMass, Mass, Position, Velocity};

#[derive(Clone)]
pub struct Body {
    pub _name: String,
    pub position: DVec2,
    pub velocity: DVec2,
    pub radius: f32,
    pub mass: f64,
}

pub fn calculate_center_of_mass_system(
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

pub fn compute_forces(bodies: &Vec<(Position, Velocity, Mass)>) -> Vec<DVec2> {
    let mut forces = vec![DVec2::zero(); bodies.len()];
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let direction = bodies[j].0 .0 - bodies[i].0 .0;
            let distance = direction.mag() + 100.0; // Avoid division by zero
            let force_magnitude = (6.6743e-11 * (bodies[i].2 .0 * bodies[j].2 .0)) / (distance * distance); // Clamp force
            let force: DVec2 = direction.normalized() * force_magnitude;
            forces[i] += force;
            forces[j] -= force; // Newton's third law: equal and opposite force
        }
    }
    forces
}

pub fn update_bodies(bodies: &mut Vec<(Position, Velocity, Mass)>, forces: Vec<DVec2>, dt: f64) {
    for (body, force) in bodies.iter_mut().zip(forces.iter()) {
        let acceleration = *force / body.2 .0;
        body.1 .0 += acceleration * dt;
        body.0 .0 += body.1 .0 * dt;
    }
}

pub fn physics_sim(bodies: &mut Vec<(Position, Velocity, Mass)>, dt: f64, precision: u32) {
    // run 10 iterations of the physics simulation, with a time step of dt/10
    for _ in 0..precision {
        let forces = compute_forces(bodies);
        update_bodies(bodies, forces, dt/precision as f64);
    }
}