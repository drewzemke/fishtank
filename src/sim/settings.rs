use crate::sim::param::Param;

pub struct Settings {
    particle_count: Param<f64>,
    gravity: Param<f64>,
    dampening: Param<f64>,
    target_density: Param<f64>,
    stiffness: Param<f64>,
    smoothing_radius: Param<f64>,
    viscosity: Param<f64>,
    mouse_force_strength: Param<f64>,
    mouse_force_radius: Param<f64>,

    selected_idx: usize,
    visible: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            particle_count: Param::default()
                .min(500.)
                .max(20000.)
                .step(500.)
                .base(10000.),
            gravity: Param::default().min(0.).max(50.).step(1.).base(15.),
            dampening: Param::default().min(0.).max(1.0).step(0.01).base(0.01),
            target_density: Param::default().min(0.1).max(10.0).step(0.1).base(1.0),
            stiffness: Param::default().min(0.).max(9000.).step(100.).base(3000.),
            smoothing_radius: Param::default().min(0.5).max(5.0).step(0.1).base(2.0),
            viscosity: Param::default().min(0.).max(20.0).step(0.1).base(2.0),
            mouse_force_strength: Param::default().min(0.).max(20.0).step(0.5).base(3.0),
            mouse_force_radius: Param::default().min(5.0).max(50.0).step(1.0).base(15.0),

            selected_idx: 0,
            visible: false,
        }
    }
}

impl Settings {
    // metadata for rendering
    pub const NAMES: [&'static str; 9] = [
        "Particles",
        "Gravity",
        "Density",
        "Viscosity",
        "Stiffness",
        "Smoothing Rad",
        "Dampening",
        "Mouse Force",
        "Mouse Radius",
    ];
    pub const PRECISIONS: [usize; 9] = [0, 1, 1, 1, 0, 1, 2, 1, 0];

    pub fn particle_count(&self) -> usize {
        (*self.particle_count.value()) as usize
    }

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
        9
    }

    pub fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    // helper methods for iteration
    pub fn params(&self) -> [&Param<f64>; 9] {
        [
            &self.particle_count,
            &self.gravity,
            &self.target_density,
            &self.viscosity,
            &self.stiffness,
            &self.smoothing_radius,
            &self.dampening,
            &self.mouse_force_strength,
            &self.mouse_force_radius,
        ]
    }

    fn params_mut(&mut self) -> [&mut Param<f64>; 9] {
        [
            &mut self.particle_count,
            &mut self.gravity,
            &mut self.target_density,
            &mut self.viscosity,
            &mut self.stiffness,
            &mut self.smoothing_radius,
            &mut self.dampening,
            &mut self.mouse_force_strength,
            &mut self.mouse_force_radius,
        ]
    }

    pub fn select_next(&mut self) {
        if !self.visible {
            return;
        }
        self.selected_idx = (self.selected_idx + 1) % Self::num_settings();
    }

    pub fn select_prev(&mut self) {
        if !self.visible {
            return;
        }
        self.selected_idx = (Self::num_settings() + self.selected_idx - 1) % Self::num_settings();
    }

    pub fn inc_selected(&mut self) {
        if !self.visible {
            return;
        }
        let idx = self.selected_idx;
        self.params_mut()[idx].inc();
    }

    pub fn dec_selected(&mut self) {
        if !self.visible {
            return;
        }
        let idx = self.selected_idx;
        self.params_mut()[idx].dec();
    }

    pub fn reset_selected(&mut self) {
        if !self.visible {
            return;
        }
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
        settings.toggle_visibility();

        assert_eq!(settings.selected_idx, 0);
        settings.select_next();
        assert_eq!(settings.selected_idx, 1);
        settings.select_prev();
        assert_eq!(settings.selected_idx, 0);
    }

    #[test]
    fn wrap_cursor() {
        let mut settings = Settings::default();
        settings.toggle_visibility();

        assert_eq!(settings.selected_idx, 0);
        settings.select_prev();
        assert_eq!(settings.selected_idx, Settings::num_settings() - 1);
        settings.select_next();
        assert_eq!(settings.selected_idx, 0);
    }
}
