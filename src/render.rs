use crate::sim::{Simulation, settings::Settings};

const DITHER_RADIUS: f64 = 0.5;

const SETTINGS_WIDTH: usize = 26;

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

                        if settings.visible()
                            && row_idx < Settings::num_settings() + 2
                            && col_idx >= self.cols - SETTINGS_WIDTH
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
        if !settings.visible() {
            return String::new();
        }

        let mut out = String::new();
        let selected = settings.selected_idx();
        let params = settings.params();

        const CONTENT_WIDTH: usize = SETTINGS_WIDTH - 2; // subtract borders

        // top border
        out.push('┌');
        out.push_str(&"─".repeat(CONTENT_WIDTH));
        out.push('┐');

        // settings rows
        const NAME_COL_WIDTH: usize = 15; // width for name column (marker + name)
        const VALUE_COL_WIDTH: usize = 7; // width for value column

        for (idx, (name, precision)) in Settings::NAMES
            .iter()
            .zip(Settings::PRECISIONS.iter())
            .enumerate()
        {
            let marker = if selected == idx { '>' } else { ' ' };

            // format value - handle particle count (idx 0) specially as integer
            let value_str = if idx == 0 {
                format!("{}", settings.particle_count())
            } else {
                let value = *params[idx].value();
                format!("{:.prec$}", value, prec = precision)
            };

            // left-align name in its column
            let name_part = format!("{} {}", marker, name);
            let name_col = format!("{:<width$}", name_part, width = NAME_COL_WIDTH);

            // right-align value in its column
            let value_col = format!("{:>width$}", value_str, width = VALUE_COL_WIDTH);

            // combine and pad to full width
            let mut line = format!("{} {}", name_col, value_col);
            if line.len() < CONTENT_WIDTH {
                line.push_str(&" ".repeat(CONTENT_WIDTH - line.len()));
            }

            out.push_str(&format!("│{}│", line));
        }

        // bottom border
        out.push('└');
        out.push_str(&"─".repeat(CONTENT_WIDTH));
        out.push('┘');

        out
    }
}
