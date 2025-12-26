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
    sim::{Simulation, runner::run_sim_loop, seed::add_uniform_points, settings::Settings},
};

fn main() -> anyhow::Result<()> {
    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    let mut stdout = stdout();

    let (cols, rows) = terminal::size().unwrap();

    let renderer = Renderer::new(rows as usize, cols as usize);

    let mut sim = Simulation::new(cols as f64, 2. * rows as f64);

    // seed the sim with random particles
    add_uniform_points(&mut sim, 10000, cols as f64, rows as f64 * 2.0);

    let sim = Arc::new(Mutex::new(sim));
    let settings = Arc::new(Mutex::new(Settings::default()));

    // used to compute framerate
    let mut frames = 0;
    let mut framerate: f64 = -1.;
    let mut frame_time = std::time::Instant::now();

    // start the sim
    let sim_clone = sim.clone();
    let settings_clone = settings.clone();
    std::thread::spawn(move || {
        run_sim_loop(sim_clone, settings_clone);
    });

    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(10))? {
            let event = crossterm::event::read()?;

            match event {
                crossterm::event::Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => {
                            // exit the program
                            break;
                        }
                        KeyCode::Char('s') => {
                            let mut settings = settings.lock().unwrap();
                            settings.toggle_visibility();
                        }
                        KeyCode::Char('r') => {
                            let mut settings = settings.lock().unwrap();
                            settings.reset_selected();
                        }
                        KeyCode::Down => {
                            let mut settings = settings.lock().unwrap();
                            settings.select_next();
                        }
                        KeyCode::Up => {
                            let mut settings = settings.lock().unwrap();
                            settings.select_prev();
                        }
                        KeyCode::Right => {
                            let mut settings = settings.lock().unwrap();
                            settings.inc_selected();
                        }
                        KeyCode::Left => {
                            let mut settings = settings.lock().unwrap();
                            settings.dec_selected();
                        }
                        _ => {}
                    }
                }
                crossterm::event::Event::Mouse(event) => {
                    let mut sim = sim.lock().unwrap();
                    match event.kind {
                        MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
                            let center = (event.column as f64, event.row as f64 * 2.);

                            match btn {
                                crossterm::event::MouseButton::Left => {
                                    sim.mouse_force.set_positive(center.0, center.1);
                                }
                                crossterm::event::MouseButton::Right => {
                                    sim.mouse_force.set_negative(center.0, center.1);
                                }
                                crossterm::event::MouseButton::Middle => {}
                            }
                        }
                        MouseEventKind::Up(..) => {
                            sim.mouse_force.reset();
                        }
                        _ => {}
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
            let settings = settings.lock().unwrap();

            let output = renderer.render(&sim, &settings);

            execute!(stdout, MoveTo(0, 0))?;
            stdout.write_all(output.as_bytes())?;

            if framerate >= 0. {
                let particle_count = format!("{} particles", sim.particles().len());
                execute!(stdout, MoveTo(0, 0))?;
                stdout.write_all(particle_count.as_bytes())?;

                let framerate = format!("{framerate:.1} FPS");
                execute!(stdout, MoveTo(0, 1))?;
                stdout.write_all(framerate.as_bytes())?;

                let framerate = format!("Sim: {:.1} ms", sim.last_frame_ms());
                execute!(stdout, MoveTo(0, 2))?;
                stdout.write_all(framerate.as_bytes())?;
            }
        }

        stdout.flush()?;
    }

    // end terminal
    execute!(stdout, Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
