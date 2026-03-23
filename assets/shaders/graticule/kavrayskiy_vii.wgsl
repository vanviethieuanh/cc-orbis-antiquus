#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> meridians: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> parallels: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> thickness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> smoothness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> color: vec4<f32>;

const PI: f32 = 3.141592653589793;
const X_MAX: f32 = 2.7206990463513265;

fn periodic_dist(x: f32) -> f32 {
    let f = fract(x);
    return min(f, 1.0 - f);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    // uv → projection space [-X_MAX, X_MAX], [-pi/2, pi/2]
    let proj_x = (uv.x - 0.5) * 2.0 * X_MAX;
    let proj_y = (uv.y - 0.5) * PI;

    // projection → geo
    let lat = proj_y;

    let lat_ratio = lat / PI;
    let sqrt_term = sqrt(1.0 / 3.0 - lat_ratio * lat_ratio);
    if sqrt_term <= 0.0 {
        discard;
    }

    let max_x = (3.0 * PI / 2.0) * sqrt_term;
    let lon = (2.0 * proj_x) / (3.0 * sqrt_term);

    // geo → normalized [0, 1]
    let lon_norm = (lon + PI) / (2.0 * PI);
    let lat_norm = (lat + PI / 2.0) / PI / 2.0;

    // grid distances
    let meridian_phase = lon_norm * (meridians + meridians * thickness * smoothness)
        + 0.5
        - meridians * thickness * 0.5 * smoothness;
    let parallel_phase = lat_norm * parallels;

    let d_lon = abs(fract(meridian_phase) - 0.5);
    let d_lat = abs(fract(parallel_phase) - 0.5);

    // distortion correction
    let d_lon_corrected = d_lon * sqrt_term;

    let aa_lon = fwidth(d_lon_corrected) * smoothness;
    let aa_lat = fwidth(d_lat) * smoothness;

    let mask_lon = 1.0 - smoothstep(thickness, thickness + aa_lon, d_lon_corrected);
    let mask_lat = 1.0 - smoothstep(thickness, thickness + aa_lat, d_lat);

    let grid_mask = max(mask_lon, mask_lat);

    // projection boundary
    let edge_dist = abs(proj_x) - max_x;
    let edge_aa = fwidth(edge_dist) * smoothness;
    let shape_mask = 1.0 - smoothstep(0.0, edge_aa, edge_dist);
    if shape_mask <= 0.0 {
        discard;
    }

    // final
    let alpha = grid_mask * shape_mask;

    return vec4<f32>(color.rgb * grid_mask, alpha * color.a);
}
