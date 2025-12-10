use rand::Rng;

const GRAVITY: f64 = 20.;

const DAMPENING: f64 = 0.9;

const COLLISION_RADIUS: f64 = 2.;

pub struct Particle {
    pos: (f64, f64),
    pub vel: (f64, f64),
}

impl Particle {
    pub fn new(x: f64, y: f64, vel_x: f64, vel_y: f64) -> Self {
        Self {
            pos: (x, y),
            vel: (vel_x, vel_y),
        }
    }

    pub fn update_pos(&mut self, dt_secs: f64) {
        self.vel.1 -= GRAVITY * dt_secs;
        self.pos.1 -= self.vel.1 * dt_secs;
        self.pos.0 -= self.vel.0 * dt_secs;
    }

    pub fn to_cell(&self) -> (usize, usize) {
        (self.y() as usize, self.x() as usize)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.pos.0
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.pos.1
    }

    #[inline]
    pub fn vel_x(&self) -> f64 {
        self.vel.0
    }

    #[inline]
    pub fn vel_y(&self) -> f64 {
        self.vel.1
    }

    #[inline]
    fn set_x(&mut self, x: f64) {
        self.pos.0 = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.pos.1 = y
    }

    pub fn compute_collision_impulse(p1: &Self, p2: &Self) -> (f64, f64) {
        // make sure they're actually colliding
        if (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2) > COLLISION_RADIUS.powi(2) {
            return (0., 0.);
        }

        // collision normal
        let n = (p2.x() - p1.x(), p2.y() - p1.y());

        // normalize it
        let mag = (n.0 * n.0 + n.1 * n.1).sqrt();
        let n = (n.0 / mag, n.1 / mag);

        // relative velocity
        let rel_vel = (p1.vel_x() - p2.vel_x(), p1.vel_y() - p2.vel_y());

        // project relative velocity onto collision vector
        let vel_along_normal = rel_vel.0 * n.0 + rel_vel.1 * n.1;

        // if particles are moving away from each other, return 0
        if vel_along_normal > 0. {
            (0., 0.)
        } else {
            (
                DAMPENING * vel_along_normal * n.0,
                DAMPENING * vel_along_normal * n.1,
            )
        }
    }
}

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
