use std::f64::consts::PI;

use crate::sim::constants::SMOOTHING_RADIUS;

// can't do SMOOTHING_RADIUS.powi(9) in const so I made a mess instead
const POLY6_COEFF: f64 = 315.
    / (64.
        * PI
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS);

pub fn poly6(sq_dist: f64) -> f64 {
    POLY6_COEFF * (SMOOTHING_RADIUS.powi(2) - sq_dist).powi(3)
}

// same
const SPIKY_GRAD_COEFF: f64 = -45.
    / (PI
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS);

pub fn spiky_grad(dist: f64) -> f64 {
    SPIKY_GRAD_COEFF * (SMOOTHING_RADIUS - dist).powi(2)
}

const VISC_LAPLACIAN_COEFF: f64 = 45.
    / (PI
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS
        * SMOOTHING_RADIUS);

pub fn visc_laplacian(dist: f64) -> f64 {
    VISC_LAPLACIAN_COEFF * (SMOOTHING_RADIUS - dist)
}
