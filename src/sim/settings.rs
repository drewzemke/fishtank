use crate::sim::param::Param;

pub struct Settings {
    gravity: Param<f64>,
    dampening: Param<f64>,

    selected_idx: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: Param::default().min(0.).max(20.).step(1.).base(15.),
            dampening: Param::default().min(0.).max(1.0).step(0.01).base(0.01),

            selected_idx: 0,
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

    pub fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    pub fn select_next(&mut self) {
        self.selected_idx = (self.selected_idx + 1) % Self::num_settings();
    }

    pub fn select_prev(&mut self) {
        self.selected_idx = (Self::num_settings() + self.selected_idx - 1) % Self::num_settings();
    }

    pub fn inc_selected(&mut self) {
        match self.selected_idx {
            0 => self.gravity.inc(),
            1 => self.dampening.inc(),
            _ => unreachable!(),
        }
    }

    pub fn dec_selected(&mut self) {
        match self.selected_idx {
            0 => self.gravity.dec(),
            1 => self.dampening.dec(),
            _ => unreachable!(),
        }
    }

    pub fn reset_selected(&mut self) {
        match self.selected_idx {
            0 => self.gravity.reset(),
            1 => self.dampening.reset(),
            _ => unreachable!(),
        }
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
