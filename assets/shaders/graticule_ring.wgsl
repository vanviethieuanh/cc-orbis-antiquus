#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> divisions: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> ring_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> thickness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> smoothness: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> divisions_smoothness: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let center = vec2<f32>(0.5, 0.5);
    let dist = uv - center;
    let d = length(dist);

    let aa = fwidth(d) * smoothness;

    let inner_radius = 0.5 - thickness; // mesh radius is 0.5
    let ring_alpha = smoothstep(inner_radius - aa, inner_radius + aa, d) -
                     smoothstep(0.5 - aa, 0.5 + aa, d);

    let angle = atan2(dist.y, dist.x); // returns [-π, π]
    let normalized_angle = (angle + 3.1415926) / 6.2831853; // map to [0,1)

    let index = normalized_angle * divisions * 0.5;
    let section = abs(fract(index + 0.25) - 0.5);

    let section_mask = smoothstep(0.25 - divisions_smoothness, 0.25 + divisions_smoothness, section);

    let final_alpha = ring_alpha * section_mask;

    return vec4(ring_color.rgb, ring_color.a * final_alpha);
}
