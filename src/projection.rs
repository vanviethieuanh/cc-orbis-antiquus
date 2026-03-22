use bevy::math::Vec2;

pub struct ProjectionResult {
    pub x: f32,
    pub y: f32,
    pub visible: bool,
}

pub fn perspective_pole(r: f32, lon: f32, lat: f32, lon0: f32, d: f32) -> ProjectionResult {
    let d_abs = r * d;

    let phi = lat.abs().to_radians();
    let lambda = lon.to_radians();
    let lambda0 = lon0.to_radians();

    let theta = lambda - lambda0;
    let k = (d_abs - r) / (d_abs - r * (phi).sin());
    let rho = r * k * phi.cos();

    let x = rho * theta.sin() + r;
    let y = -rho * theta.cos() + r;

    ProjectionResult {
        x,
        y,
        visible: phi.sin() >= r / d_abs,
    }
}

pub fn max_projected_radius(r: f32, d: f32) -> f32 {
    let distance = d * r;

    let sin_phi = r / distance;
    let phi = sin_phi.asin();

    let k = (distance - r) / (distance - r * phi.sin());
    let rho = r * k * phi.cos();

    rho
}

pub fn parallel_ratio(lat_deg: f32, r: f32, d: f32) -> f32 {
    let phi = lat_deg.to_radians();
    let d_abs = d * r;

    let rho_phi = r * ((d_abs - r) / (d_abs - r * phi.sin())) * phi.cos();
    let rho_max = max_projected_radius(r, d);

    rho_phi / rho_max
}

pub fn eckert_iv_project(lat: f32, lon: f32) -> Vec2 {
    let phi = lat;
    let lambda = lon;

    let c = 2.0 + std::f32::consts::PI / 2.0;

    let mut theta = phi;

    for _ in 0..6 {
        let sin_t = theta.sin();
        let cos_t = theta.cos();

        let f = theta + sin_t * cos_t + 2.0 * sin_t - c * phi.sin();
        let df = 1.0 + cos_t * cos_t - sin_t * sin_t + 2.0 * cos_t;

        theta -= f / df;
    }

    let kx = 2.0 / (4.0 * std::f32::consts::PI + std::f32::consts::PI.powi(2)).sqrt();
    let ky = 2.0 * std::f32::consts::PI.sqrt() / (4.0 + std::f32::consts::PI).sqrt();

    let x = kx * lambda * (1.0 + theta.cos());
    let y = ky * theta.sin();

    Vec2::new(x, y)
}
