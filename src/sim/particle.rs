use super::constants::{COLLISION_RADIUS, DAMPENING, GRAVITY};

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
