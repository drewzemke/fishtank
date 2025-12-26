use std::collections::HashMap;

use rayon::prelude::*;

use particle::Particle;

use crate::sim::{
    constants::{
        CELL_SIZE, MOUSE_FORCE_RADIUS, MOUSE_FORCE_STRENGTH, PARTICLE_MASS, SMOOTHING_RADIUS,
        SMOOTHING_RADIUS_SQ, VISCOSITY,
    },
    kernels::{poly6, spiky_grad, visc_laplacian},
    settings::Settings,
};

mod constants;
mod kernels;
mod param;
mod particle;
pub mod runner;
pub mod seed;
pub mod settings;

type GridPoint = (i64, i64);

pub enum MouseForce {
    Positive { x: f64, y: f64 },
    Negative { x: f64, y: f64 },
    None,
}

impl MouseForce {
    pub fn reset(&mut self) {
        *self = Self::None
    }

    pub fn set_positive(&mut self, x: f64, y: f64) {
        *self = Self::Positive { x, y }
    }

    pub fn set_negative(&mut self, x: f64, y: f64) {
        *self = Self::Negative { x, y }
    }
}

pub struct Simulation {
    width: f64,
    height: f64,

    particles: Vec<Particle>,

    // FIXME: make this private
    pub mouse_force: MouseForce,

    last_frame_ms: f64,
}

impl Simulation {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            particles: Vec::new(),
            mouse_force: MouseForce::None,
            last_frame_ms: 0.,
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(x, y, 0., 0.));
    }

    pub fn update(&mut self, dt_secs: f64, settings: &Settings) {
        let start_time = std::time::Instant::now();

        // hash particles into a grid
        let (keys, spatial_hash) = self.build_hash();

        // density computation
        let densities = self.compute_densities(&keys, &spatial_hash);

        // pressure computation
        let target_density = settings.target_density();
        let stiffness = settings.stiffness();
        let pressures = densities
            .iter()
            .map(|d| (stiffness * (d - target_density)).max(0.))
            .collect::<Vec<_>>();

        // force computation
        let forces = self.compute_forces(keys, spatial_hash, &densities, pressures, settings);

        // apply forces to move particles
        self.apply_forces(dt_secs, densities, forces);

        // apply boundaries
        self.apply_boundaries(settings);

        // compute time
        let time = start_time.elapsed().as_secs_f64();
        self.last_frame_ms = time * 1000.;
    }

    fn build_hash(&mut self) -> (Vec<GridPoint>, HashMap<GridPoint, Vec<usize>>) {
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
        (keys, spatial_hash)
    }

    fn compute_densities(
        &mut self,
        keys: &[(i64, i64)],
        spatial_hash: &HashMap<(i64, i64), Vec<usize>>,
    ) -> Vec<f64> {
        self.particles
            .par_iter()
            .enumerate()
            .map(|(idx1, pt)| {
                let key = keys[idx1];
                let mut density = 0.;

                // only do computations in neighboring cells
                for x_offset in [-1, 0, 1] {
                    for y_offset in [-1, 0, 1] {
                        let key = (key.0 + x_offset, key.1 + y_offset);
                        if let Some(v) = spatial_hash.get(&key) {
                            for idx2 in v {
                                let pt2 = &self.particles[*idx2];

                                // restrict attention to neighbors within SMOOTHING_RADIUS
                                let sq_dist =
                                    (pt.x() - pt2.x()).powi(2) + (pt.y() - pt2.y()).powi(2);
                                if sq_dist > SMOOTHING_RADIUS_SQ {
                                    continue;
                                }

                                density += PARTICLE_MASS * poly6(sq_dist);
                            }
                        }
                    }
                }

                density
            })
            .collect::<Vec<_>>()
    }

    fn compute_forces(
        &mut self,
        keys: Vec<(i64, i64)>,
        spatial_hash: HashMap<(i64, i64), Vec<usize>>,
        densities: &[f64],
        pressures: Vec<f64>,
        settings: &Settings,
    ) -> Vec<(f64, f64)> {
        let gravity = settings.gravity();

        self.particles
            .par_iter()
            .enumerate()
            .map(|(idx1, pt)| {
                let key = keys[idx1];
                let mut force = (0., -gravity);

                // only do computations in neighboring cells
                for x_offset in [-1, 0, 1] {
                    for y_offset in [-1, 0, 1] {
                        let key = (key.0 + x_offset, key.1 + y_offset);
                        if let Some(v) = spatial_hash.get(&key) {
                            for idx2 in v {
                                let pt2 = &self.particles[*idx2];

                                // restrict attention to neighbors within SMOOTHING_RADIUS, excluding self,
                                let disp = (pt.x() - pt2.x(), pt.y() - pt2.y());
                                let dist = (disp.0.powi(2) + disp.1.powi(2)).sqrt();
                                if idx1 == *idx2 || dist > SMOOTHING_RADIUS || dist <= 0. {
                                    continue;
                                }

                                // pressure force
                                let pressure_force_coeff = PARTICLE_MASS
                                    * (pressures[idx1] + pressures[*idx2])
                                    * spiky_grad(dist)
                                    / (2. * densities[*idx2] * dist);

                                force.0 += pressure_force_coeff * disp.0;
                                force.1 += pressure_force_coeff * disp.1;

                                // viscosity force
                                let vel_diff = (pt2.vel_x() - pt.vel_x(), pt2.vel_y() - pt.vel_y());

                                let visc_force_coeff =
                                    VISCOSITY * PARTICLE_MASS * visc_laplacian(dist)
                                        / densities[*idx2];

                                force.0 += visc_force_coeff * vel_diff.0;
                                force.1 += visc_force_coeff * vel_diff.1;
                            }
                        }
                    }
                }

                // include mouse force
                match self.mouse_force {
                    MouseForce::Positive { x, y } => {
                        let disp = (x - pt.x(), y - pt.y());
                        let dist = (disp.0.powi(2) + disp.1.powi(2)).sqrt();
                        let coeff = MOUSE_FORCE_STRENGTH * (MOUSE_FORCE_RADIUS - dist).max(0.)
                            / densities[idx1];

                        // positive pressue => push away from the center
                        force.0 += coeff * disp.0;
                        force.1 += coeff * disp.1;
                    }
                    MouseForce::Negative { x, y } => {
                        let disp = (x - pt.x(), y - pt.y());
                        let dist = (disp.0.powi(2) + disp.1.powi(2)).sqrt();
                        let coeff = MOUSE_FORCE_STRENGTH * (MOUSE_FORCE_RADIUS - dist).max(0.)
                            / densities[idx1];

                        // negative pressue => push towards the center
                        force.0 -= coeff * disp.0;
                        force.1 -= coeff * disp.1;

                        // also push lightly against the velocity of the particle if it's close to the center
                        // this stops the particles from oscillating wildly around the ball
                        if dist < MOUSE_FORCE_RADIUS {
                            force.0 -= coeff / 30.0 * pt.vel_x();
                            force.1 -= coeff / 30.0 * pt.vel_y();
                        }
                    }
                    MouseForce::None => {}
                }

                force
            })
            .collect::<Vec<_>>()
    }

    fn apply_forces(&mut self, dt_secs: f64, densities: Vec<f64>, forces: Vec<(f64, f64)>) {
        for (idx, p) in self.particles.iter_mut().enumerate() {
            let density = densities[idx];
            let force = (forces[idx].0 / density, forces[idx].1 / density);
            p.update_vel(force, dt_secs);
            p.update_pos(dt_secs);
        }
    }

    fn apply_boundaries(&mut self, settings: &Settings) {
        let width = self.width;
        let height = self.height;
        let dampening = settings.dampening();

        for particle in self.particles.iter_mut() {
            if particle.x() < 0. {
                particle.set_x(-particle.x());
                particle.vel.0 *= -dampening;
            }

            if particle.y() < 0. {
                particle.set_y(-particle.y());
                particle.vel.1 *= -dampening;
            }

            if particle.x() > width {
                particle.set_x(width - (particle.x() - width));
                particle.vel.0 *= -dampening;
            }

            if particle.y() > height {
                particle.set_y(height - (particle.y() - height));
                particle.vel.1 *= -dampening;
            }
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn last_frame_ms(&self) -> f64 {
        self.last_frame_ms
    }
}
