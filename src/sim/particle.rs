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

    pub fn update_vel(&mut self, force: (f64, f64), dt_secs: f64) {
        self.vel.0 += force.0 * dt_secs;
        self.vel.1 += force.1 * dt_secs;
    }

    pub fn update_pos(&mut self, dt_secs: f64) {
        self.pos.1 -= self.vel.1 * dt_secs;
        self.pos.0 -= self.vel.0 * dt_secs;
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
    pub fn set_x(&mut self, x: f64) {
        self.pos.0 = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.pos.1 = y
    }
}
