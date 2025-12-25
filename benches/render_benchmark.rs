use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use fishtank::{
    render::Renderer,
    sim::{Simulation, seed::add_uniform_points, settings::Settings},
};

fn render_benches(c: &mut Criterion) {
    c.bench_function("renderer 10000 particles at 80x40", |b| {
        let mut sim = Simulation::new(80., 40.);
        let settings = Settings::default();
        add_uniform_points(&mut sim, 10_000, 80., 40.);

        let renderer = Renderer::new(80, 40);

        b.iter(|| {
            let s = renderer.render(&sim, &settings);
            black_box(s);
        })
    });
}

criterion_group!(benches, render_benches);
criterion_main!(benches);
