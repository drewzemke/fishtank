use std::{
    io::{Write, stdout},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crossterm::{cursor::MoveTo, execute};

use crate::{
    render::Renderer,
    sim::{Simulation, settings::Settings},
};

const TARGET_FPS: f64 = 60.0;
const TARGET_FRAME_TIME: Duration = Duration::from_micros((1_000_000.0 / TARGET_FPS) as u64);
const SLEEP_OVERHEAD: Duration = Duration::from_millis(3); // compensate for OS sleep overhead

pub fn run_render_loop(
    sim: Arc<Mutex<Simulation>>,
    settings: Arc<Mutex<Settings>>,
    renderer: Arc<Mutex<Renderer>>,
) {
    let mut stdout = stdout();
    let mut frames = 0;
    let mut framerate: f64 = -1.;
    let mut frame_time = Instant::now();
    let mut render_time_ms: f64 = 0.;

    loop {
        let frame_start = Instant::now();

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
            let renderer = renderer.lock().unwrap();

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

                let render_time_str = format!("Render: {:.1} ms", render_time_ms);
                execute!(stdout, MoveTo(0, 3)).unwrap();
                stdout.write_all(render_time_str.as_bytes()).unwrap();
            }
        }

        stdout.flush().unwrap();

        render_time_ms = frame_start.elapsed().as_secs_f64() * 1000.0;
        std::thread::sleep(TARGET_FRAME_TIME.saturating_sub(frame_start.elapsed() + SLEEP_OVERHEAD));
    }
}
