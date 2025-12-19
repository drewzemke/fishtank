pub const TIMESTEP_MS: u64 = 20;

pub const GRAVITY: f64 = 20.;

// collision model
pub const DAMPENING: f64 = 0.01;

pub const COLLISION_RADIUS: f64 = 1.;

// fluid sim
pub const SMOOTHING_RADIUS: f64 = 2.;

pub const PARTICLE_MASS: f64 = 1.;

pub const TARGET_DENSITY: f64 = 0.5;

pub const STIFFNESS: f64 = 200.;

pub const VISCOSITY: f64 = 0.5;

// spatial hashing
pub const CELL_SIZE: f64 = SMOOTHING_RADIUS;
