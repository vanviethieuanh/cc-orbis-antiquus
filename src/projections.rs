use std::f32::consts::PI;

pub fn draw_pole(r: f32, long: f32, lat: f32, d: f32) -> (f32, f32) {
    let d_abs = d * r;
    let lat_rad = lat.to_radians();
    let theta = long.to_radians();

    let mut rho = r * (d_abs - r) / (d_abs - r * lat_rad.sin()) * lat_rad.cos();

    let x = rho * theta.sin() + r;
    let y = -rho * theta.cos() + r;

    (x, y)
}
