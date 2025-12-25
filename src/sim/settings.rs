use crate::sim::param::Param;

pub struct Settings {
    gravity: Param<f64>,
    dampening: Param<f64>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: Param::default().min(0.).max(20.).step(1.).base(15.),
            dampening: Param::default().min(0.).max(1.0).step(0.01).base(0.01),
        }
    }
}

impl Settings {
    pub fn gravity(&self) -> f64 {
        *self.gravity.value()
    }

    pub fn dampening(&self) -> f64 {
        *self.dampening.value()
    }

    pub const fn num_settings() -> usize {
        2
    }
}
