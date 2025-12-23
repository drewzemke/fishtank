use criterion::{Criterion, criterion_group, criterion_main};
use fishtank::sim::{Simulation, seed::add_uniform_points};

fn sim_benches(c: &mut Criterion) {
    c.bench_function("simulate 10000 particles at 80x40", |b| {
        let mut sim = Simulation::new(80., 40.);
        add_uniform_points(&mut sim, 10_000, 80., 40.);

        b.iter(|| {
            sim.update(0.02);
        })
    });
}

criterion_group!(benches, sim_benches);
criterion_main!(benches);
