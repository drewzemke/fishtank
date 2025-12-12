use std::{
    f64::consts::PI,
    io::{Write, stdout},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fishtank::{render::Renderer, sim::Simulation};

fn main() -> anyhow::Result<()> {
    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    let mut stdout = stdout();

    let (cols, rows) = terminal::size().unwrap();

    let mut sim = Simulation::new(cols as f64, 2. * rows as f64);
    let mut renderer = Renderer::new(rows as usize, cols as usize);

    // used to compute dt
    let mut time = std::time::Instant::now();

    // used to compute framerate
    let mut frames = 0;
    let mut framerate: f64 = -1.;
    let mut frame_time = std::time::Instant::now();

    loop {
        let dt = time.elapsed();
        time = std::time::Instant::now();

        if crossterm::event::poll(std::time::Duration::from_millis(20))? {
            let event = crossterm::event::read()?;

            match event {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        // exit the program
                        break;
                    }
                }
                crossterm::event::Event::Mouse(event) => {
                    if matches!(event.kind, MouseEventKind::Down(..)) {
                        for i in -10..=10 {
                            for j in -10..=10 {
                                sim.add_particle(
                                    event.column as f64 + i as f64,
                                    event.row as f64 + j as f64,
                                );
                            }
                        }

                        let r = 10.;
                        let n = 50;
                        for i in 0..n {
                            let i = i as f64;
                            sim.add_particle(
                                event.column as f64 + r * f64::cos(2. * i * PI / n as f64),
                                2. * event.row as f64 + r * f64::sin(2. * i * PI / n as f64),
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // update framerate every 100 frames
        frames += 1;
        if frames % 100 == 0 {
            let time = frame_time.elapsed();
            framerate = 100.0 / time.as_secs_f64();
            frame_time = std::time::Instant::now();
        }

        // render

        sim.update(dt.as_secs_f64());
        let output = renderer.render(&sim);

        execute!(stdout, MoveTo(0, 0))?;
        stdout.write_all(output.as_bytes())?;

        if framerate >= 0. {
            let framerate = format!("{framerate:.1} FPS");
            execute!(stdout, MoveTo(0, 0))?;
            stdout.write_all(framerate.as_bytes())?;

            let particle_count = format!("{} particles", sim.particles().len());
            execute!(stdout, MoveTo(0, 1))?;
            stdout.write_all(particle_count.as_bytes())?;
        }

        stdout.flush()?;
    }

    // end terminal
    execute!(stdout, Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
