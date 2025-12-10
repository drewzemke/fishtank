use rand::Rng;

const GRAVITY: f64 = 20.;

const DAMPENING: f64 = 0.8;

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
        for particle in &mut self.particles {
            particle.update_pos(dt_secs);

            // if the particle is out of bounds, put it back in bounds reverse its velocity with dampening
            if particle.x() < 0. {
                particle.set_x(-particle.x());
                particle.vel.0 *= -DAMPENING;
            }

            if particle.y() < 0. {
                particle.set_y(-particle.y());
                particle.vel.1 *= -DAMPENING;
            }

            if particle.x() > self.width {
                particle.set_x(self.width - (particle.x() - self.width));
                particle.vel.0 *= -DAMPENING;
            }

            if particle.y() > self.height {
                particle.set_y(self.height - (particle.y() - self.height));
                particle.vel.1 *= -DAMPENING;
            }
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
