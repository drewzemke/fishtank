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

    let (cols, rows) = terminal::size().unwrap();

    let renderer = Renderer::new(rows as usize, cols as usize);

    let mut sim = Simulation::new(cols as f64, 2. * rows as f64);
    let settings = Settings::default();

    // seed the sim with random particles
    add_uniform_points(&mut sim, settings.particle_count(), cols as f64, rows as f64 * 2.0);

    let sim = Arc::new(Mutex::new(sim));
    let settings = Arc::new(Mutex::new(settings));
    let renderer = Arc::new(Mutex::new(renderer));

    // start the sim thread
    let sim_clone = sim.clone();
    let settings_clone = settings.clone();
    std::thread::spawn(move || {
        run_sim_loop(sim_clone, settings_clone);
    });

    // start the render thread
    let sim_clone = sim.clone();
    let settings_clone = settings.clone();
    let renderer_clone = renderer.clone();
    std::thread::spawn(move || {
        let mut stdout = stdout();
        let mut frames = 0;
        let mut framerate: f64 = -1.;
        let mut frame_time = std::time::Instant::now();

        loop {
            // cap at ~60 fps
            std::thread::sleep(std::time::Duration::from_millis(16));

            // update framerate every 100 frames
            frames += 1;
            if frames % 100 == 0 {
                let time = frame_time.elapsed();
                framerate = 100.0 / time.as_secs_f64();
                frame_time = std::time::Instant::now();
            }

            // render
            {
                let sim = sim_clone.lock().unwrap();
                let settings = settings_clone.lock().unwrap();
                let renderer = renderer_clone.lock().unwrap();

                let output = renderer.render(&sim, &settings);

                execute!(stdout, MoveTo(0, 0)).unwrap();
                stdout.write_all(output.as_bytes()).unwrap();

                if framerate >= 0. {
                    let particle_count = format!("{} particles", sim.particles().len());
                    execute!(stdout, MoveTo(0, 0)).unwrap();
                    stdout.write_all(particle_count.as_bytes()).unwrap();

                    let framerate_str = format!("{framerate:.1} FPS");
                    execute!(stdout, MoveTo(0, 1)).unwrap();
                    stdout.write_all(framerate_str.as_bytes()).unwrap();

                    let sim_time_str = format!("Sim: {:.1} ms", sim.last_frame_ms());
                    execute!(stdout, MoveTo(0, 2)).unwrap();
                    stdout.write_all(sim_time_str.as_bytes()).unwrap();
                }
            }

            stdout.flush().unwrap();
        }
    });

    loop {
        // blocking wait for events - no need to poll at high rate
        let event = crossterm::event::read()?;

        match event {
            crossterm::event::Event::Resize(cols, rows) => {
                let mut renderer = renderer.lock().unwrap();
                renderer.resize(rows as usize, cols as usize);
                let mut sim = sim.lock().unwrap();
                sim.resize(cols as f64, 2. * rows as f64);
            }
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
                            let target_count = settings.particle_count();
                            drop(settings);
                            let mut sim = sim.lock().unwrap();
                            sim.sync_particle_count(target_count);
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
                            let target_count = settings.particle_count();
                            drop(settings);
                            let mut sim = sim.lock().unwrap();
                            sim.sync_particle_count(target_count);
                        }
                        KeyCode::Left => {
                            let mut settings = settings.lock().unwrap();
                            settings.dec_selected();
                            let target_count = settings.particle_count();
                            drop(settings);
                            let mut sim = sim.lock().unwrap();
                            sim.sync_particle_count(target_count);
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

    // end terminal
    execute!(stdout(), Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
