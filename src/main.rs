use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

const GRAVITY: f64 = 20.;

struct Particle {
    x: f64,
    y: f64,
    vel: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, vel: 0. }
    }

    pub fn update(&mut self, dt_secs: f64) {
        self.vel -= GRAVITY * dt_secs;
        self.y -= self.vel * dt_secs;
    }

    pub fn to_cell(&self) -> (u16, u16) {
        (self.y as u16, self.x as u16)
    }
}

fn main() -> anyhow::Result<()> {
    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    let mut stdout = stdout();

    let mut particles = Vec::new();

    let (cols, rows) = terminal::size().unwrap();

    // used to compute dt
    let mut time = std::time::Instant::now();

    loop {
        let dt = time.elapsed();
        time = std::time::Instant::now();

        if crossterm::event::poll(std::time::Duration::from_millis(20))? {
            let event = crossterm::event::read()?;

            match event {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                crossterm::event::Event::Mouse(event) => {
                    if matches!(event.kind, MouseEventKind::Down(..)) {
                        let particle = Particle::new(event.column as f64, event.row as f64);
                        particles.push(particle);
                    }
                }
                _ => {}
            }
        }

        execute!(stdout, MoveTo(0, 0))?;

        let mut output = vec![vec![' '; cols as usize]; rows as usize];

        for particle in &mut particles {
            particle.update(dt.as_secs_f64());
            let (row, col) = particle.to_cell();
            let row = row.min(rows - 1) as usize;
            let col = col.min(cols - 1) as usize;
            output[row][col] = '█';
        }

        let output = output.into_iter().flatten().collect::<String>();

        stdout.write_all(output.as_bytes())?;
        stdout.flush()?;
    }

    // end terminal
    execute!(stdout, Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
