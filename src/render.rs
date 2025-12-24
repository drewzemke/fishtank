use crate::sim::Simulation;

const DITHER_RADIUS: f64 = 0.5;

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
        let mut output = vec![vec![Some(0u8); self.cols]; self.rows];

        for (i, particle) in sim.particles().iter().enumerate() {
            // spatial dithering: add small deterministic offset to break up moiré patterns
            // hash-based offset ensures no flickering while disrupting grid alignment
            let hash = i.wrapping_mul(2654435761) ^ (i >> 16);
            let dx = ((hash & 0xFF) as f64 / 255.0 - 0.5) * DITHER_RADIUS;
            let dy = (((hash >> 8) & 0xFF) as f64 / 255.0 - 0.5) * DITHER_RADIUS;

            let x = particle.x() + dx;
            let y = particle.y() + dy;

            let row = y as usize / 2;
            let col = x as usize;

            if row >= self.rows || col >= self.cols {
                continue;
            }

            // the braille unicode character 0x28XX puts dots based on the bits of the
            // 'XX' bytes, according to this layout:
            //
            // 0 3
            // 1 4
            // 2 5
            // 6 7   <- annoying bottom row
            //
            // so we use the position of the particle within the cell to
            // compute which row/column of that grid it's in, then OR that bit
            // into this cell's running value
            let x_half = (x.fract() >= 0.5) as u8;
            let y_quarter = (y / 2.).fract() * 4.0;
            let bit = (y_quarter as u8) + x_half * 3;
            let bit = if y_quarter >= 3. { 6 + x_half } else { bit };

            output[row][col] = Some(output[row][col].unwrap_or(0) | 1 << bit);
        }

        self.last = output
            .into_iter()
            .flatten()
            .map(|byte| match byte {
                None => ' ',
                Some(b) => {
                    let v = 0x2800u32 | (b as u32);
                    char::from_u32(v).unwrap_or(' ')
                } // _ => {
            })
            .collect::<String>();

        &self.last
    }
}
