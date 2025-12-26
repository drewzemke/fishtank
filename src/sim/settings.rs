use crate::sim::param::Param;

pub struct Settings {
    gravity: Param<f64>,
    dampening: Param<f64>,
    target_density: Param<f64>,
    stiffness: Param<f64>,

    selected_idx: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: Param::default().min(0.).max(20.).step(1.).base(15.),
            dampening: Param::default().min(0.).max(1.0).step(0.01).base(0.01),
            target_density: Param::default().min(0.1).max(2.0).step(0.1).base(1.0),
            stiffness: Param::default().min(0.).max(5000.).step(100.).base(2000.),

            selected_idx: 0,
        }
    }
}

impl Settings {
    // metadata for rendering
    pub const NAMES: [&'static str; 4] = ["gravity", "dampening", "density", "stiffness"];
    pub const PRECISIONS: [usize; 4] = [1, 2, 1, 0];

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

    pub const fn num_settings() -> usize {
        4
    }

    pub fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    // helper methods for iteration
    pub fn params(&self) -> [&Param<f64>; 4] {
        [&self.gravity, &self.dampening, &self.target_density, &self.stiffness]
    }

    fn params_mut(&mut self) -> [&mut Param<f64>; 4] {
        [&mut self.gravity, &mut self.dampening, &mut self.target_density, &mut self.stiffness]
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
