use std::{
    io::stdout,
    sync::{Arc, Mutex},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fishtank::{
    render::{Renderer, info::Info, runner::run_render_loop},
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
    let info = Arc::new(Mutex::new(Info::default()));

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
    let info_clone = info.clone();
    std::thread::spawn(move || {
        run_render_loop(sim_clone, settings_clone, renderer_clone, info_clone);
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
                        KeyCode::Char('i') => {
                            let mut info = info.lock().unwrap();
                            info.toggle_visibility();
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
