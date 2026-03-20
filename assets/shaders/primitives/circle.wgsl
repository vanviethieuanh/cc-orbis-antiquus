#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> border_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> fill_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> border_thickness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> smoothness: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let center = vec2<f32>(0.5, 0.5);
    let d = distance(uv, center);

    let radius = 0.5;
    let aa = fwidth(d) * smoothness;

    let fill_alpha = 1.0 - smoothstep(radius - aa, radius + aa, d);

    let inner_radius = radius - border_thickness;
    let border_alpha = smoothstep(inner_radius - aa, inner_radius + aa, d) -
                       smoothstep(radius - aa, radius + aa, d);

    let fill = fill_color * fill_alpha;
    let border = border_color * border_alpha;

    return fill + border;
}
