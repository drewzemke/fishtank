pub const TIMESTEP_MS: u64 = 5;

// fluid sim
pub const SMOOTHING_RADIUS: f64 = 2.;

pub const SMOOTHING_RADIUS_SQ: f64 = SMOOTHING_RADIUS * SMOOTHING_RADIUS;

pub const PARTICLE_MASS: f64 = 1.;

pub const TARGET_DENSITY: f64 = 1.0;

pub const STIFFNESS: f64 = 2000.;

pub const VISCOSITY: f64 = 2.0;

// mouse interaction
pub const MOUSE_FORCE_STRENGTH: f64 = 3.;

pub const MOUSE_FORCE_RADIUS: f64 = 15.;

// spatial hashing
pub const CELL_SIZE: f64 = 0.9 * SMOOTHING_RADIUS;
