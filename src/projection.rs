use std::f32::consts::PI;

use bevy::math::Vec2;

pub fn perspective_polar_projection_clamped(coor: Vec2, d: f32) -> Vec2 {
    let r = 1.0;

    let view_pole_sign = d.signum();
    let d_abs = r * d.abs();
    let phi = coor.y.to_radians();
    let lambda = coor.x.to_radians();
    let sin_limit = r / d_abs;
    let theta = lambda;

    if phi.sin() * view_pole_sign < sin_limit {
        let cos_limit = (1.0 - sin_limit * sin_limit).sqrt();
        let k_limit = (d_abs - r) / (d_abs - r * sin_limit);
        let rho_max = r * k_limit * cos_limit;
        let dir = Vec2::new(theta.sin(), -theta.cos());
        let p = dir * rho_max + Vec2::splat(r);

        return Vec2::new(p.x, p.y);
    }

    let k = (d_abs - r) / (d_abs - r * (phi).sin());
    let rho = r * k * phi.cos();

    let x = rho * theta.sin() + r;
    let y = -rho * theta.cos() + r;

    Vec2::new(x, y)
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

/// Projects geographic coordinates (longitude and latitude) to the Kavrayskiy VII projection.
///
/// # Parameters
/// - `lon_deg`: Longitude in **degrees** (typical range: -180 to 180)
/// - `lat_deg`: Latitude in **degrees** (typical range: -90 to 90)
///
/// # Returns
/// - `(x, y)`: Projected coordinates in radians-equivalent units
///
/// # Projection Ranges
/// - `x ∈ [-π√3/2, π√3/2] ≈ [-2.7207, 2.7207]`
/// - `y ∈ [-π/2, π/2] ≈ [-1.5708, 1.5708]`
///
/// # Example
/// ```
/// let (x, y) = kavrayskiy_vii(0.0, 0.0);
/// println!("Projected: x = {}, y = {}", x, y);
/// ```
pub fn kavrayskiy_vii(coor: Vec2) -> Vec2 {
    let lon = coor.x.to_radians();
    let lat = coor.y.to_radians();

    Vec2::new(1.5 * lon * ((1.0 / 3.0) - (lat / PI).powi(2)).sqrt(), lat)
}
