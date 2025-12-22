pub const TIMESTEP_MS: u64 = 10;

pub const GRAVITY: f64 = 2.;

// collision model
pub const DAMPENING: f64 = 0.01;

pub const COLLISION_RADIUS: f64 = 1.;

// fluid sim
pub const SMOOTHING_RADIUS: f64 = 2.;

pub const SMOOTHING_RADIUS_SQ: f64 = SMOOTHING_RADIUS * SMOOTHING_RADIUS;

pub const PARTICLE_MASS: f64 = 1.;

pub const TARGET_DENSITY: f64 = 0.5;

// FIXME: this is used more like an inverse stiffness
pub const STIFFNESS: f64 = 300.;

pub const VISCOSITY: f64 = 1.;

// spatial hashing
pub const CELL_SIZE: f64 = SMOOTHING_RADIUS;
