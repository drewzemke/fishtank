use rand::Rng;

use crate::sim::Simulation;

pub fn add_dense_square(sim: &mut Simulation, center: (f64, f64), radius: i32) {
    for i in -radius..=radius {
        for j in -radius..=radius {
            sim.add_particle(center.0 + (i as f64) / 2., center.1 + (j as f64) / 2.);
        }
    }
}

pub fn add_uniform_points(sim: &mut Simulation, count: usize, width: f64, height: f64) {
    let mut rng = rand::rng();

    for _ in 0..count {
        let x = rng.random_range(0.0..width);
        let y = rng.random_range(0.0..height);
        sim.add_particle(x, y);
    }
}
