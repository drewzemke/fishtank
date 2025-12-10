const GRAVITY: f64 = 20.;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vel: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, vel: 0. }
    }

    pub fn update(&mut self, dt_secs: f64) {
        self.vel -= GRAVITY * dt_secs;
        self.y -= self.vel * dt_secs;
    }

    pub fn to_cell(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}

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
        self.particles.push(Particle::new(x, y));
    }

    pub fn update(&mut self, dt_secs: f64) {
        for particle in &mut self.particles {
            particle.update(dt_secs);
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
