use std::collections::HashMap;

use constants::DAMPENING;
use particle::Particle;

use crate::sim::{
    constants::{
        CELL_SIZE, GRAVITY, PARTICLE_MASS, SMOOTHING_RADIUS, SMOOTHING_RADIUS_SQ, STIFFNESS,
        TARGET_DENSITY, VISCOSITY,
    },
    kernels::{poly6, spiky_grad, visc_laplacian},
};

mod constants;
mod kernels;
mod particle;
pub mod runner;
pub mod seed;

pub struct Simulation {
    width: f64,
    height: f64,
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(x, y, 0., 0.));
    }

    pub fn update(&mut self, dt_secs: f64) {
        let keys = self
            .particles
            .iter()
            .map(|particle| {
                (
                    (particle.x() / CELL_SIZE).floor() as i64,
                    (particle.y() / CELL_SIZE).floor() as i64,
                )
            })
            .collect::<Vec<_>>();

        // hash particle positions into cells
        let mut spatial_hash: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (idx, key) in keys.iter().enumerate() {
            spatial_hash
                .entry(*key)
                .and_modify(|v| v.push(idx))
                .or_insert_with(|| Vec::from([idx]));
        }

        // density computation
        let mut densities = vec![0.; self.particles.len()];

        for (idx1, pt) in self.particles.iter().enumerate() {
            let key = keys[idx1];

            // only do computations in neighboring cells
            for x_offset in [-1, 0, 1] {
                for y_offset in [-1, 0, 1] {
                    let key = (key.0 + x_offset, key.1 + y_offset);
                    if let Some(v) = spatial_hash.get(&key) {
                        for idx2 in v {
                            let pt2 = &self.particles[*idx2];

                            // restrict attention to neighbors within SMOOTHING_RADIUS
                            let sq_dist = (pt.x() - pt2.x()).powi(2) + (pt.y() - pt2.y()).powi(2);
                            if sq_dist > SMOOTHING_RADIUS_SQ {
                                continue;
                            }

                            densities[idx1] += PARTICLE_MASS * poly6(sq_dist);
                        }
                    }
                }
            }
        }

        // pressure computation
        let pressures = densities
            .iter()
            .map(|d| STIFFNESS * (TARGET_DENSITY - d))
            .collect::<Vec<_>>();

        // force computation
        let mut forces = vec![(0., -GRAVITY); self.particles.len()];

        for (idx1, pt) in self.particles.iter().enumerate() {
            let key = keys[idx1];

            // only do computations in neighboring cells
            for x_offset in [-1, 0, 1] {
                for y_offset in [-1, 0, 1] {
                    let key = (key.0 + x_offset, key.1 + y_offset);
                    if let Some(v) = spatial_hash.get(&key) {
                        for idx2 in v {
                            let pt2 = &self.particles[*idx2];

                            // restrict attention to neighbors within SMOOTHING_RADIUS, excluding self,
                            // NOTE: using newton's third law, we can also eliminate
                            // the case where idx1 > idx2, and assign the force for
                            // idx2 as the negative of the force at idx1
                            let disp = (pt.x() - pt2.x(), pt.y() - pt2.y());
                            let dist = (disp.0.powi(2) + disp.1.powi(2)).sqrt();
                            if idx1 >= *idx2 || dist > SMOOTHING_RADIUS || dist <= 0. {
                                continue;
                            }

                            // pressure force
                            let pressure_force_coeff = -PARTICLE_MASS
                                * (pressures[idx1] + pressures[*idx2])
                                * spiky_grad(dist)
                                / (2. * densities[*idx2] * dist);

                            forces[idx1].0 += pressure_force_coeff * disp.0;
                            forces[idx1].1 += pressure_force_coeff * disp.1;
                            forces[*idx2].0 -= pressure_force_coeff * disp.0;
                            forces[*idx2].1 -= pressure_force_coeff * disp.1;

                            // viscosity force
                            let vel_diff = (pt2.vel_x() - pt.vel_x(), pt2.vel_y() - pt.vel_y());

                            let visc_force_coeff =
                                VISCOSITY * PARTICLE_MASS * visc_laplacian(dist) / densities[*idx2];

                            forces[idx1].0 += visc_force_coeff * vel_diff.0;
                            forces[idx1].1 += visc_force_coeff * vel_diff.1;
                            forces[*idx2].0 -= visc_force_coeff * vel_diff.0;
                            forces[*idx2].1 -= visc_force_coeff * vel_diff.1;
                        }
                    }
                }
            }
        }

        // apply forces and move particles
        for (idx, p) in self.particles.iter_mut().enumerate() {
            let density = densities[idx];
            let force = (forces[idx].0 / density, forces[idx].1 / density);
            p.update_vel(force, dt_secs);
            p.update_pos(dt_secs);
            Self::apply_boundaries(p, self.width, self.height);
        }
    }

    fn apply_boundaries(particle: &mut Particle, width: f64, height: f64) {
        if particle.x() < 0. {
            particle.set_x(-particle.x());
            particle.vel.0 *= -DAMPENING;
        }

        if particle.y() < 0. {
            particle.set_y(-particle.y());
            particle.vel.1 *= -DAMPENING;
        }

        if particle.x() > width {
            particle.set_x(width - (particle.x() - width));
            particle.vel.0 *= -DAMPENING;
        }

        if particle.y() > height {
            particle.set_y(height - (particle.y() - height));
            particle.vel.1 *= -DAMPENING;
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
