use crate::sim::Simulation;

pub struct Renderer {
    last: String,
    rows: usize,
    cols: usize,
}

impl Renderer {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            last: String::new(),
        }
    }

    pub fn render(&mut self, sim: &Simulation) -> &str {
        let mut output = vec![vec![' '; self.cols]; self.rows];

        for particle in sim.particles() {
            let (row, col) = particle.to_cell();
            let row = row.min(self.rows - 1);
            let col = col.min(self.cols - 1);
            output[row][col] = '█';
        }

        self.last = output.into_iter().flatten().collect::<String>();

        &self.last
    }
}
