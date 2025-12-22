pub const TIMESTEP_MS: u64 = 5;

pub const GRAVITY: f64 = 15.;

// collision model
pub const DAMPENING: f64 = 0.01;

pub const COLLISION_RADIUS: f64 = 1.;

// fluid sim
pub const SMOOTHING_RADIUS: f64 = 2.;

pub const SMOOTHING_RADIUS_SQ: f64 = SMOOTHING_RADIUS * SMOOTHING_RADIUS;

pub const PARTICLE_MASS: f64 = 1.;

pub const TARGET_DENSITY: f64 = 1.0;

pub const STIFFNESS: f64 = 2000.;

pub const VISCOSITY: f64 = 2.0;

// mouse interaction
pub const MOUSE_FORCE_STRENGTH: f64 = 7.;

pub const MOUSE_FORCE_RADIUS: f64 = 15.;

// spatial hashing
pub const CELL_SIZE: f64 = SMOOTHING_RADIUS;
