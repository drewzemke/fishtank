use std::sync::{Arc, Mutex};

use crate::sim::{Simulation, constants::TIMESTEP_MS, settings::Settings};

pub fn run_sim_loop(sim: Arc<Mutex<Simulation>>, settings: Arc<Mutex<Settings>>) {
    let mut time = std::time::Instant::now();

    loop {
        let dt = time.elapsed();
        time = std::time::Instant::now();

        {
            let mut sim = sim.lock().unwrap();
            let settings = settings.lock().unwrap();
            sim.update(dt.as_secs_f64(), &settings);
        }

        // FIXME: need to take processing time into account here
        std::thread::sleep(std::time::Duration::from_millis(TIMESTEP_MS));
    }
}
