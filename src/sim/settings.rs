use crate::sim::param::Param;

pub struct Settings {
    gravity: Param<f64>,
    dampening: Param<f64>,
    target_density: Param<f64>,
    stiffness: Param<f64>,
    smoothing_radius: Param<f64>,
    viscosity: Param<f64>,
    mouse_force_strength: Param<f64>,
    mouse_force_radius: Param<f64>,

    selected_idx: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: Param::default().min(0.).max(20.).step(1.).base(15.),
            dampening: Param::default().min(0.).max(1.0).step(0.01).base(0.01),
            target_density: Param::default().min(0.1).max(2.0).step(0.1).base(1.0),
            stiffness: Param::default().min(0.).max(5000.).step(100.).base(2000.),
            smoothing_radius: Param::default().min(0.5).max(5.0).step(0.1).base(2.0),
            viscosity: Param::default().min(0.).max(10.0).step(0.1).base(2.0),
            mouse_force_strength: Param::default().min(0.).max(10.0).step(0.5).base(3.0),
            mouse_force_radius: Param::default().min(5.0).max(50.0).step(1.0).base(15.0),

            selected_idx: 0,
        }
    }
}

impl Settings {
    // metadata for rendering
    pub const NAMES: [&'static str; 8] = [
        "gravity",
        "dampening",
        "density",
        "stiffness",
        "smooth_r",
        "viscosity",
        "mouse_str",
        "mouse_r",
    ];
    pub const PRECISIONS: [usize; 8] = [1, 2, 1, 0, 1, 1, 1, 0];

    pub fn gravity(&self) -> f64 {
        *self.gravity.value()
    }

    pub fn dampening(&self) -> f64 {
        *self.dampening.value()
    }

    pub fn target_density(&self) -> f64 {
        *self.target_density.value()
    }

    pub fn stiffness(&self) -> f64 {
        *self.stiffness.value()
    }

    pub fn smoothing_radius(&self) -> f64 {
        *self.smoothing_radius.value()
    }

    pub fn viscosity(&self) -> f64 {
        *self.viscosity.value()
    }

    pub fn mouse_force_strength(&self) -> f64 {
        *self.mouse_force_strength.value()
    }

    pub fn mouse_force_radius(&self) -> f64 {
        *self.mouse_force_radius.value()
    }

    // computed values
    pub fn smoothing_radius_sq(&self) -> f64 {
        let r = self.smoothing_radius();
        r * r
    }

    pub fn cell_size(&self) -> f64 {
        0.9 * self.smoothing_radius()
    }

    pub const fn num_settings() -> usize {
        8
    }

    pub fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    // helper methods for iteration
    pub fn params(&self) -> [&Param<f64>; 8] {
        [
            &self.gravity,
            &self.dampening,
            &self.target_density,
            &self.stiffness,
            &self.smoothing_radius,
            &self.viscosity,
            &self.mouse_force_strength,
            &self.mouse_force_radius,
        ]
    }

    fn params_mut(&mut self) -> [&mut Param<f64>; 8] {
        [
            &mut self.gravity,
            &mut self.dampening,
            &mut self.target_density,
            &mut self.stiffness,
            &mut self.smoothing_radius,
            &mut self.viscosity,
            &mut self.mouse_force_strength,
            &mut self.mouse_force_radius,
        ]
    }

    pub fn select_next(&mut self) {
        self.selected_idx = (self.selected_idx + 1) % Self::num_settings();
    }

    pub fn select_prev(&mut self) {
        self.selected_idx = (Self::num_settings() + self.selected_idx - 1) % Self::num_settings();
    }

    pub fn inc_selected(&mut self) {
        let idx = self.selected_idx;
        self.params_mut()[idx].inc();
    }

    pub fn dec_selected(&mut self) {
        let idx = self.selected_idx;
        self.params_mut()[idx].dec();
    }

    pub fn reset_selected(&mut self) {
        let idx = self.selected_idx;
        self.params_mut()[idx].reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_cursor() {
        let mut settings = Settings::default();

        assert_eq!(settings.selected_idx, 0);
        settings.select_next();
        assert_eq!(settings.selected_idx, 1);
        settings.select_prev();
        assert_eq!(settings.selected_idx, 0);
    }

    #[test]
    fn wrap_cursor() {
        let mut settings = Settings::default();

        assert_eq!(settings.selected_idx, 0);
        settings.select_prev();
        assert_eq!(settings.selected_idx, Settings::num_settings() - 1);
        settings.select_next();
        assert_eq!(settings.selected_idx, 0);
    }
}
