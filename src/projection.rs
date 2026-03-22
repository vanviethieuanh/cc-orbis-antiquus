use std::f32::consts::PI;

use std::ops::RangeInclusive;

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
pub fn kavrayskiy_vii(lon_deg: f32, lat_deg: f32) -> (f32, f32) {
    let lon = lon_deg.to_radians();
    let lat = lat_deg.to_radians();

    let x = 1.5 * lon * ((1.0 / 3.0) - (lat / PI).powi(2)).sqrt();
    let y = lat;

    (x, y)
}

const KAVRAYSKIY_VII_X_RANGE: RangeInclusive<f32> = -2.7207..=2.7207;
const KAVRAYSKIY_VII_Y_RANGE: RangeInclusive<f32> = -1.5708..=1.5708;
