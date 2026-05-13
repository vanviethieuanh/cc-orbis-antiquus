#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ParchmentTextureParams {
    // vec4 fields
    parchment_bg: vec4<f32>,
    parchment_light: vec4<f32>,
    parchment_dark: vec4<f32>,
    foxing_color: vec4<f32>,
    stain_color: vec4<f32>,
    // vec2 fields
    mesh_ratio: vec2<f32>,
    fiber_scale_h: vec2<f32>,
    fiber_scale_v: vec2<f32>,
    stain1_edge: vec2<f32>,
    stain2_edge: vec2<f32>,
    patch_edge: vec2<f32>,
    // f32 fields
    base_scale: f32,
    base_blend: f32,
    grain_scale: f32,
    grain_strength: f32,
    fiber_h_weight: f32,
    fiber_strength: f32,
    foxing_voronoi_scale: f32,
    foxing_noise_scale: f32,
    foxing_edge_low: f32,
    foxing_edge_high: f32,
    foxing_threshold: f32,
    foxing_strength: f32,
    stain1_scale: f32,
    stain2_scale: f32,
    stain_strength: f32,
    patch_scale: f32,
    patch_strength: f32,
    line_freq: f32,
    line_width: f32,
    line_var_scale_y: f32,
    line_shade_factor: f32,
    line_strength: f32,
    vignette_strength: f32,
    dither_scale: f32,
    dither_strength: f32,
    // i32 fields
    base_octaves: i32,
    grain_octaves: i32,
    fiber_octaves_h: i32,
    fiber_octaves_v: i32,
    foxing_noise_octaves: i32,
    stain1_octaves: i32,
    stain2_octaves: i32,
    patch_octaves: i32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: ParchmentTextureParams;

// --- Noise helpers ---
fn hash2(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453);
}

fn hash2v(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(
        fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453),
        fract(sin(dot(p, vec2<f32>(269.5, 183.3))) * 43758.5453),
    );
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let a = hash2(i + vec2<f32>(0.0, 0.0));
    let b = hash2(i + vec2<f32>(1.0, 0.0));
    let c = hash2(i + vec2<f32>(0.0, 1.0));
    let d = hash2(i + vec2<f32>(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm(p_in: vec2<f32>, octaves: i32) -> f32 {
    var p = p_in;
    var value = 0.0;
    var amplitude = 0.5;
    for (var i = 0; i < octaves; i = i + 1) {
        value += amplitude * value_noise(p);
        p *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}

// Voronoi-based cellular noise for age spots / foxing
fn voronoi(p: vec2<f32>) -> f32 {
    let n = floor(p);
    let f = fract(p);
    var min_dist = 1.0;
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let point = hash2v(n + neighbor);
            let diff = neighbor + point - f;
            let dist = length(diff);
            min_dist = min(min_dist, dist);
        }
    }
    return min_dist;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv * params.mesh_ratio;

    // ---- Base paper color with large-scale variation ----
    let large_variation = fbm(uv * params.base_scale + 42.0, params.base_octaves);
    var paper = mix(params.parchment_bg, params.parchment_light, large_variation * params.base_blend);

    // ---- Fine fiber / grain texture (horizontal bias like real paper fibers) ----
    let fiber_h = fbm(vec2<f32>(uv.x * params.fiber_scale_h.x, uv.y * params.fiber_scale_h.y) + 7.0, params.fiber_octaves_h);
    let fiber_v = fbm(vec2<f32>(uv.x * params.fiber_scale_v.x, uv.y * params.fiber_scale_v.y) + 13.0, params.fiber_octaves_v);
    let fiber = fiber_h * params.fiber_h_weight + fiber_v * (1.0 - params.fiber_h_weight);
    paper = mix(paper, params.parchment_dark, fiber * params.fiber_strength);

    // ---- Medium grain / paper texture ----
    let grain = fbm(uv * params.grain_scale + 17.0, params.grain_octaves);
    paper = mix(paper, params.parchment_light, (grain - 0.5) * params.grain_strength);

    // ---- Age spots / foxing (small brown spots scattered) ----
    let foxing_cells = voronoi(uv * params.foxing_voronoi_scale + 100.0);
    let foxing_noise = fbm(uv * params.foxing_noise_scale + 55.0, params.foxing_noise_octaves);
    let foxing_mask = (1.0 - smoothstep(params.foxing_edge_low, params.foxing_edge_high, foxing_cells)) * step(params.foxing_threshold, foxing_noise);
    paper = mix(paper, params.foxing_color, foxing_mask * params.foxing_strength);

    // Larger, fainter age stains
    let stain1 = fbm(uv * params.stain1_scale + 88.0, params.stain1_octaves);
    let stain2 = fbm(uv * params.stain2_scale + 200.0, params.stain2_octaves);
    let stain_mask = smoothstep(params.stain1_edge.x, params.stain1_edge.y, stain1) * smoothstep(params.stain2_edge.x, params.stain2_edge.y, stain2);
    paper = mix(paper, params.stain_color, stain_mask * params.stain_strength);

    // ---- Discoloration patches ----
    let patch_noise = fbm(uv * params.patch_scale + 300.0, params.patch_octaves);
    let patch_mask = smoothstep(params.patch_edge.x, params.patch_edge.y, patch_noise);
    paper = mix(paper, params.parchment_light, patch_mask * params.patch_strength);

    // ---- Bamboo tray lines (dense vertical lines from paper-making mold) ----
    let line_pos = fract(uv.x * params.line_freq);
    // Thin line with soft edges
    let line_base = 1.0 - smoothstep(0.0, params.line_width, abs(line_pos - 0.5));
    // Vary intensity along y so lines aren't perfectly uniform
    let line_variation = value_noise(vec2<f32>(uv.x * params.line_freq * 0.5, uv.y * params.line_var_scale_y));
    let line_mask = line_base * mix(0.5, 1.0, line_variation);
    // Each line randomly picks between light and dark, biased toward light
    let line_shade = hash2(vec2<f32>(floor(uv.x * params.line_freq), 0.0));
    let line_color = mix(params.parchment_light, params.parchment_dark, line_shade * params.line_shade_factor);
    paper = mix(paper, line_color, line_mask * params.line_strength);

    // ---- Edge darkening ----
    let center = mesh.uv - vec2<f32>(0.5);
    let vignette = 1.0 - dot(center, center) * params.vignette_strength;
    paper = vec4<f32>(paper.rgb * vignette, 1.0);

    // ---- Micro noise to break banding ----
    let dither = (hash2(mesh.uv * params.dither_scale) - 0.5) * params.dither_strength;
    paper = vec4<f32>(paper.rgb + dither, 1.0);

    return paper;
}
