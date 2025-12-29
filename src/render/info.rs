const INFO_WIDTH: usize = 20;

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
    pub const fn render_width() -> usize {
        INFO_WIDTH
    }

    pub const fn render_height() -> usize {
        6 // border + 4 lines + border
    }

    pub fn update(
        &mut self,
        particle_count: usize,
        sim_time_ms: f64,
        render_time_ms: f64,
        fps: f64,
    ) {
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

    pub fn render(&self) -> String {
        if !self.visible {
            return String::new();
        }

        let mut out = String::new();
        const CONTENT_WIDTH: usize = INFO_WIDTH - 2;

        // top border
        out.push('┌');
        out.push_str(&"─".repeat(CONTENT_WIDTH));
        out.push('┐');

        // info rows
        let lines = [
            format!("Particles: {}", self.particle_count),
            format!("Sim: {:.1} ms", self.sim_time_ms),
            format!("Render: {:.1} ms", self.render_time_ms),
            format!("FPS: {:.1}", self.fps),
        ];

        for line in &lines {
            let mut padded = line.clone();
            if padded.len() < CONTENT_WIDTH {
                padded.push_str(&" ".repeat(CONTENT_WIDTH - padded.len()));
            }
            out.push_str(&format!("│{}│", padded));
        }

        // bottom border
        out.push('└');
        out.push_str(&"─".repeat(CONTENT_WIDTH));
        out.push('┘');

        out
    }
}
