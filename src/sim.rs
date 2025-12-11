use rand::Rng;

use constants::DAMPENING;
use particle::Particle;

mod constants;
mod particle;

pub struct Simulation {
    width: f64,
    height: f64,
    particles: Vec<Particle>,
    rng: rand::rngs::ThreadRng,
}

impl Simulation {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            particles: Vec::new(),
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
        for idx1 in 0..self.particles.len() {
            for idx2 in (idx1 + 1)..self.particles.len() {
                let [p1, p2] = self
                    .particles
                    .get_disjoint_mut([idx1, idx2])
                    .expect("valid indices");

                let impulse = Particle::compute_collision_impulse(p1, p2);
                p1.vel = (p1.vel.0 - impulse.0, p1.vel.1 - impulse.1);
                p2.vel = (p2.vel.0 + impulse.0, p2.vel.1 + impulse.1);
            }

            let particle = &mut self.particles[idx1];
            Self::compute_boundary_collisions(particle, self.width, self.height);
            particle.update_pos(dt_secs);
        }
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
