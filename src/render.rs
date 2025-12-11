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
        let mut output = vec![vec![0u8; self.cols]; self.rows];

        for particle in sim.particles() {
            let row = particle.y() as usize / 2;
            let col = particle.x() as usize;

            if row >= self.rows || col >= self.cols {
                continue;
            }

            if (particle.y() / 2.).fract() > 0.5 {
                output[row][col] |= 0b10; // bottom half
            } else {
                output[row][col] |= 0b01; // top
            }
        }

        self.last = output
            .into_iter()
            .flatten()
            .map(|byte| match byte {
                0 => ' ',
                1 => '▀',
                2 => '▄',
                3 => '█',
                _ => unreachable!(),
            })
            .collect::<String>();

        &self.last
    }
}
