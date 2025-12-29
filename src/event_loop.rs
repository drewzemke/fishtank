use std::sync::{Arc, Mutex};

use crossterm::event::{self, KeyCode, MouseEventKind};

use crate::{
    render::{Renderer, info::Info},
    sim::{Simulation, settings::Settings},
};

pub fn run_event_loop(
    sim: Arc<Mutex<Simulation>>,
    settings: Arc<Mutex<Settings>>,
    renderer: Arc<Mutex<Renderer>>,
    info: Arc<Mutex<Info>>,
) -> anyhow::Result<()> {
    loop {
        // blocking wait for events - no need to poll at high rate
        let event = event::read()?;

        match event {
            event::Event::Resize(cols, rows) => {
                let mut renderer = renderer.lock().unwrap();
                renderer.resize(rows as usize, cols as usize);
                let mut sim = sim.lock().unwrap();
                sim.resize(cols as f64, 2. * rows as f64);
            }
            event::Event::Key(event) => {
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
            event::Event::Mouse(event) => {
                let mut sim = sim.lock().unwrap();
                match event.kind {
                    MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
                        let center = (event.column as f64, event.row as f64 * 2.);

                        match btn {
                            event::MouseButton::Left => {
                                sim.mouse_force.set_positive(center.0, center.1);
                            }
                            event::MouseButton::Right => {
                                sim.mouse_force.set_negative(center.0, center.1);
                            }
                            event::MouseButton::Middle => {}
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

    Ok(())
}
