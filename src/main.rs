use std::{
    io::{Write, stdout},
    sync::{Arc, Mutex},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fishtank::{
    render::Renderer,
    sim::{
        Simulation,
        runner::run_sim_loop,
        seed::{add_dense_square, add_uniform_points},
    },
};

fn main() -> anyhow::Result<()> {
    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    let mut stdout = stdout();

    let (cols, rows) = terminal::size().unwrap();

    let mut renderer = Renderer::new(rows as usize, cols as usize);

    let mut sim = Simulation::new(cols as f64, 2. * rows as f64);

    // seed the sim with random particles
    add_uniform_points(&mut sim, 10000, cols as f64, rows as f64 * 2.0);

    let sim = Arc::new(Mutex::new(sim));

    // used to compute framerate
    let mut frames = 0;
    let mut framerate: f64 = -1.;
    let mut frame_time = std::time::Instant::now();

    // start the sim
    let sim_clone = sim.clone();
    std::thread::spawn(move || {
        run_sim_loop(sim_clone);
    });

    loop {
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
                    let mut sim = sim.lock().unwrap();
                    if matches!(event.kind, MouseEventKind::Down(..)) {
                        let center = (event.column as f64, event.row as f64 * 2.);
                        add_dense_square(&mut sim, center, 20);
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
        {
            let sim = sim.lock().unwrap();

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
        }

        stdout.flush()?;
    }

    // end terminal
    execute!(stdout, Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
