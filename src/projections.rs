pub struct ProjectionResult {
    pub x: f32,
    pub y: f32,
    pub visible: bool,
}

pub fn perspective_pole(r: f32, lon: f32, lat: f32, d: f32) -> ProjectionResult {
    let lat_rad = lat.to_radians();
    let theta = lon.to_radians();

    let d_abs = d * r;

    let visible = lat_rad.sin() >= r / d_abs;

    let k = (d_abs - r) / (d_abs - r * lat_rad.sin());
    let rho = r * k * lat_rad.cos();

    let x = rho * theta.sin() + r;
    let y = -rho * theta.cos() + r;

    ProjectionResult { x, y, visible }
}
