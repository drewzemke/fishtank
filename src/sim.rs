use std::collections::HashMap;

use rand::Rng;

use constants::DAMPENING;
use particle::Particle;

use crate::sim::constants::CELL_SIZE;

mod constants;
mod particle;

pub struct Simulation {
    width: f64,
    height: f64,
    particles: Vec<Particle>,
    spatial_hash: HashMap<(i64, i64), Vec<usize>>, // maps cell to indices
    rng: rand::rngs::ThreadRng,
}

impl Simulation {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            particles: Vec::new(),
            spatial_hash: HashMap::new(),
            rng: rand::rng(),
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(
            x,
            y,
            self.rng.random_range(-50. ..50.),
            self.rng.random_range(-50. ..50.),
        ));
    }

    pub fn update(&mut self, dt_secs: f64) {
        // hash particle positions into cells
        for (idx, particle) in self.particles.iter().enumerate() {
            let key = (
                (particle.x() / CELL_SIZE).floor() as i64,
                (particle.y() / CELL_SIZE).floor() as i64,
            );
            self.spatial_hash
                .entry(key)
                .and_modify(|v| v.push(idx))
                .or_insert_with(|| Vec::from([idx]));
        }

        // for each cell, only compute collisions with nearby neighbors
        for idx1 in 0..self.particles.len() {
            let particle = &self.particles[idx1];
            let key = (
                (particle.x() / CELL_SIZE).floor() as i64,
                (particle.y() / CELL_SIZE).floor() as i64,
            );

            for x_offset in [-1, 0, 1] {
                for y_offset in [-1, 0, 1] {
                    let key = (key.0 + x_offset, key.1 + y_offset);
                    if let Some(v) = self.spatial_hash.get(&key) {
                        for idx2 in v {
                            if *idx2 <= idx1 {
                                continue;
                            }

                            let [p1, p2] = self
                                .particles
                                .get_disjoint_mut([idx1, *idx2])
                                .expect("valid indices");

                            let impulse = Particle::compute_collision_impulse(p1, p2);
                            p1.vel = (p1.vel.0 - impulse.0, p1.vel.1 - impulse.1);
                            p2.vel = (p2.vel.0 + impulse.0, p2.vel.1 + impulse.1);
                        }
                    }
                }
            }

            let particle = &mut self.particles[idx1];
            Self::compute_boundary_collisions(particle, self.width, self.height);
            particle.update_pos(dt_secs);
        }

        self.spatial_hash.drain();
    }

    fn compute_boundary_collisions(particle: &mut Particle, width: f64, height: f64) {
        // if the particle is out of bounds, put it back in bounds reverse its velocity with dampening
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
