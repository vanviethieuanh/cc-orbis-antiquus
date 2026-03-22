#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> meridians: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> parallels: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> thickness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> smoothness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> color: vec4<f32>;

const PI: f32 = 3.141592653589793;
const X_MAX: f32 = 2.7206990463513265; // π√3 / 2

fn periodic_dist(x: f32) -> f32 {
    let f = fract(x);
    return min(f, 1.0 - f);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    // ----------------------------------------
    // 1. UV → projected space
    // ----------------------------------------
    let x = (uv.x - 0.5) * 2.0 * X_MAX; // [-X_MAX, X_MAX]
    let y = (uv.y - 0.5) * PI;          // [-π/2, π/2]

    let lat = y;

    // ----------------------------------------
    // 2. projection validity + shape
    // ----------------------------------------
    let sqrt_term = sqrt(1.0 / 3.0 - (lat / PI) * (lat / PI));
    if sqrt_term <= 0.0 {
        discard;
    }

    let max_x = (3.0 * PI / 2.0) * sqrt_term;

    // mask out corners (projection boundary)
    let edge = abs(x) - max_x;
    let shape_mask = 1.0 - smoothstep(0.0, 0.002, edge);

    if shape_mask <= 0.0 {
        discard;
    }

    // ----------------------------------------
    // 3. inverse projection → lon/lat
    // ----------------------------------------
    let lon = (2.0 * x) / (3.0 * sqrt_term);

    // normalize to [0,1]
    let lon_norm = (lon + PI) / (2.0 * PI);
    let lat_norm = (lat + PI / 2.0) / PI;

    // ----------------------------------------
    // 4. graticule distances (in spherical space)
    // ----------------------------------------
    let d_lon = abs(fract(lon_norm * meridians + 0.5) - 0.5);
    let d_lat = abs(fract(lat_norm * parallels) - 0.5);

    // ----------------------------------------
    // 5. fix meridian width distortion
    // ----------------------------------------
    // horizontal scale shrinks toward poles → compensate
    let d_lon_corr = d_lon * sqrt_term;

    // combine (draw both meridians + parallels)
    let d = min(d_lon_corr, d_lat);

    // ----------------------------------------
    // 6. anti-aliasing + thickness control
    // ----------------------------------------
    let aa = fwidth(d) * smoothness;

    let grid = 1.0 - smoothstep(thickness, thickness + aa, d);

    // ----------------------------------------
    // 7. final color
    // ----------------------------------------
    let alpha = grid * shape_mask;

    return vec4<f32>(color.rgb * grid, alpha * color.a);
}
