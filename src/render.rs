use crate::sim::{Simulation, settings::Settings};

const DITHER_RADIUS: f64 = 0.5;

const SETTINGS_WIDTH: usize = 20;

pub struct Renderer {
    rows: usize,
    cols: usize,
}

impl Renderer {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn render(&self, sim: &Simulation, settings: &Settings) -> String {
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

        let settings_render = Self::render_settings(settings);

        output
            .into_iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.into_iter().enumerate().map({
                    // FIXME: any way to avoid this??
                    let settings_render = settings_render.clone();

                    move |(col_idx, byte)| {
                        let var_name = match byte {
                            None => ' ',
                            Some(b) => {
                                let v = 0x2800u32 | (b as u32);
                                char::from_u32(v).unwrap_or(' ')
                            }
                        };

                        if row_idx < Settings::num_settings()
                            && col_idx > self.cols - SETTINGS_WIDTH
                        {
                            settings_render
                                .chars()
                                .nth(
                                    row_idx * SETTINGS_WIDTH + col_idx - self.cols + SETTINGS_WIDTH,
                                )
                                .unwrap_or('X')
                        } else {
                            var_name
                        }
                    }
                })
            })
            .collect::<String>()
    }

    fn render_settings(settings: &Settings) -> String {
        let mut out = String::new();

        // FIXME: can't use SETTINGS_WIDTH here?
        out.push_str(&format!(
            "{:>20}",
            format!("gravity: {:.1}", settings.gravity())
        ));

        out.push_str(&format!(
            "{:>20}",
            format!("dampening: {:.2}", settings.dampening())
        ));

        out
    }
}
