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
            let adjusted_y = particle.y() / 2.;

            let row = adjusted_y as usize;
            let col = particle.x() as usize;

            if row >= self.rows || col >= self.cols {
                continue;
            }

            output[row][col] = if adjusted_y.fract() > 0.5 {
                if output[row][col] == '▀' {
                    '█'
                } else {
                    '▄'
                }
            } else if output[row][col] == '▄' {
                '█'
            } else {
                '▀'
            }
        }

        self.last = output.into_iter().flatten().collect::<String>();

        &self.last
    }
}
