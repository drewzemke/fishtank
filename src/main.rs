use std::{
    io::stdout,
    sync::{Arc, Mutex},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fishtank::{
    event_loop::run_event_loop,
    render::{Renderer, info::Info, runner::run_render_loop},
    sim::{Simulation, runner::run_sim_loop, seed::add_uniform_points, settings::Settings},
};

fn main() -> anyhow::Result<()> {
    let (cols, rows) = terminal::size().unwrap();

    let renderer = Renderer::new(rows as usize, cols as usize);

    let mut sim = Simulation::new(cols as f64, 2. * rows as f64);
    let settings = Settings::default();

    // start terminal
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All))?;

    // seed the sim with random particles
    add_uniform_points(
        &mut sim,
        settings.particle_count(),
        cols as f64,
        rows as f64 * 2.0,
    );

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

    // start the event loop (in this thread)
    run_event_loop(sim, settings, renderer, info)?;

    // end terminal
    execute!(stdout(), Show, DisableMouseCapture, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
