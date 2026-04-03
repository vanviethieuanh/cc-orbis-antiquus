use bevy::{log::info, math::Vec2};
use std::f32::consts::PI;

pub fn azimuthal_equidistant_clipped(ring: &[Vec2], sign: f32, lat_limit: f32) -> Vec<Vec2> {
    let r = 1.0;

    let phi0 = lat_limit.to_radians();
    let rho_max = r * (std::f32::consts::FRAC_PI_2 - sign * phi0);

    let project = |coor: &Vec2| {
        let phi = coor.y.to_radians();
        let lambda = coor.x.to_radians();

        let rho = r * (std::f32::consts::FRAC_PI_2 - sign * phi);

        let x = rho * lambda.sin();
        let y = -rho * lambda.cos();

        (Vec2::new(x, y), rho)
    };

    let intersect = |a: Vec2, b: Vec2| -> Vec2 {
        let phi_a = a.y.to_radians();
        let phi_b = b.y.to_radians();

        let rho_a = r * (std::f32::consts::FRAC_PI_2 - sign * phi_a);
        let rho_b = r * (std::f32::consts::FRAC_PI_2 - sign * phi_b);

        let t = (rho_max - rho_a) / (rho_b - rho_a);

        let lat = a.y + (b.y - a.y) * t;
        let lon = a.x + (b.x - a.x) * t;

        Vec2::new(lon, lat)
    };

    let mut result = Vec::new();
    let mut last_exit: Option<Vec2> = None;

    let start_inside = match ring.first() {
        Some(start_coor) => {
            let (_, rho_s) = project(start_coor);
            rho_s <= rho_max
        }
        None => true,
    };

    for window in ring.windows(2) {
        let a = window[0];
        let b = window[1];

        let (pa, rho_a) = project(&a);
        let (pb, rho_b) = project(&b);

        // South pole have noises
        if pa.x.abs() < 1e-5 && sign == -1.0 {
            continue;
        }

        let inside_a = rho_a <= rho_max;
        let inside_b = rho_b <= rho_max;

        match (inside_a, inside_b) {
            (true, true) => {
                if result.is_empty() {
                    result.push(pa);
                }
                result.push(pb);
            }

            (true, false) => {
                let i = intersect(a, b);
                let (pi, _) = project(&i);

                result.push(pa);
                result.push(pi);

                if start_inside {
                    last_exit = Some(i);
                } else {
                    if let Some(prev) = last_exit {
                        add_arc(&mut result, i.x, prev.x, lat_limit, &project);
                        last_exit = None;
                    }
                }
            }

            (false, true) => {
                let i = intersect(a, b);

                if start_inside {
                    if let Some(prev) = last_exit {
                        add_arc(&mut result, prev.x, i.x, lat_limit, &project);
                    }
                    last_exit = None;
                } else {
                    last_exit = Some(i);
                }

                let (pi, _) = project(&i);

                result.push(pi);
                result.push(pb);
            }

            (false, false) => {}
        }
    }

    result
}

fn add_arc(
    result: &mut Vec<Vec2>,
    lon_a: f32,
    lon_b: f32,
    lat: f32,
    project: &impl Fn(&Vec2) -> (Vec2, f32),
) {
    let mut dlon = lon_b - lon_a;

    if dlon > 180.0 {
        dlon -= 360.0;
    }
    if dlon < -180.0 {
        dlon += 360.0;
    }

    let steps = ((dlon.abs() / 5.0).ceil() as usize).max(2);

    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let lon = lon_a + dlon * t;

        let (p, _) = project(&Vec2::new(lon, lat));
        result.push(p);
    }
}

pub fn parallel_ratio(lat_deg: f32) -> f32 {
    const R: f32 = 1.0;
    let phi = lat_deg.to_radians();
    let c = std::f32::consts::FRAC_PI_2 - phi;
    R * c
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
pub fn kavrayskiy_vii(c: Vec2) -> Vec2 {
    let lon = c.x.to_radians();
    let lat = c.y.to_radians();

    let base = ((1.0 / 3.0) - (lat / PI).powi(2)).sqrt();

    // global widening
    let global = 1.1;

    // pole widening
    let t = lat.abs() / (PI / 2.0);
    let pole = 1.0 + 0.2 * t.powi(3);

    Vec2::new(1.5 * lon * base * global * pole, lat)
}

pub fn kavrayskiy_vii_ring(ring: &[Vec2]) -> Vec<Vec2> {
    ring.iter().map(|&c| kavrayskiy_vii(c)).collect()
}
