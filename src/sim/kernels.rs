use std::f64::consts::PI;

pub fn poly6(sq_dist: f64, smoothing_radius: f64, smoothing_radius_sq: f64) -> f64 {
    let coeff = 315. / (64. * PI * smoothing_radius.powi(9));
    coeff * (smoothing_radius_sq - sq_dist).powi(3)
}

pub fn spiky_grad(dist: f64, smoothing_radius: f64) -> f64 {
    let coeff = -45. / (PI * smoothing_radius.powi(6));
    coeff * (smoothing_radius - dist).powi(2)
}

pub fn visc_laplacian(dist: f64, smoothing_radius: f64) -> f64 {
    let coeff = 45. / (PI * smoothing_radius.powi(6));
    coeff * (smoothing_radius - dist)
}
