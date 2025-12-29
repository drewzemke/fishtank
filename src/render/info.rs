pub struct Info {
    particle_count: usize,
    sim_time_ms: f64,
    render_time_ms: f64,
    fps: f64,
    visible: bool,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            particle_count: 0,
            sim_time_ms: 0.,
            render_time_ms: 0.,
            fps: 0.,
            visible: false,
        }
    }
}

impl Info {
    pub fn update(&mut self, particle_count: usize, sim_time_ms: f64, render_time_ms: f64, fps: f64) {
        self.particle_count = particle_count;
        self.sim_time_ms = sim_time_ms;
        self.render_time_ms = render_time_ms;
        self.fps = fps;
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn particle_count(&self) -> usize {
        self.particle_count
    }

    pub fn sim_time_ms(&self) -> f64 {
        self.sim_time_ms
    }

    pub fn render_time_ms(&self) -> f64 {
        self.render_time_ms
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}
