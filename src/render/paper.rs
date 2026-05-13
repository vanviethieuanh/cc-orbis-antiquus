use bevy::{
    math::{Vec2, Vec4},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::Material2d,
};

#[derive(ShaderType, Debug, Clone)]
pub struct ParchmentTextureParams {
    // --- vec4 fields (16-byte aligned) ---
    pub parchment_bg: Vec4,
    pub parchment_light: Vec4,
    pub parchment_dark: Vec4,
    pub foxing_color: Vec4,
    pub stain_color: Vec4,

    // --- vec2 fields (8-byte aligned) ---
    pub mesh_ratio: Vec2,
    pub fiber_scale_h: Vec2,
    pub fiber_scale_v: Vec2,
    pub stain1_edge: Vec2,
    pub stain2_edge: Vec2,
    pub patch_edge: Vec2,

    // --- f32 fields (4-byte aligned) ---
    pub base_scale: f32,
    pub base_blend: f32,
    pub grain_scale: f32,
    pub grain_strength: f32,
    pub fiber_h_weight: f32,
    pub fiber_strength: f32,
    pub foxing_voronoi_scale: f32,
    pub foxing_noise_scale: f32,
    pub foxing_edge_low: f32,
    pub foxing_edge_high: f32,
    pub foxing_threshold: f32,
    pub foxing_strength: f32,
    pub stain1_scale: f32,
    pub stain2_scale: f32,
    pub stain_strength: f32,
    pub patch_scale: f32,
    pub patch_strength: f32,
    pub line_freq: f32,
    pub line_width: f32,
    pub line_var_scale_y: f32,
    pub line_shade_factor: f32,
    pub line_strength: f32,
    pub vignette_strength: f32,
    pub dither_scale: f32,
    pub dither_strength: f32,

    // --- i32 fields (4-byte aligned) ---
    pub base_octaves: i32,
    pub grain_octaves: i32,
    pub fiber_octaves_h: i32,
    pub fiber_octaves_v: i32,
    pub foxing_noise_octaves: i32,
    pub stain1_octaves: i32,
    pub stain2_octaves: i32,
    pub patch_octaves: i32,
}

impl Default for ParchmentTextureParams {
    fn default() -> Self {
        Self {
            parchment_bg: Vec4::new(0.910, 0.870, 0.750, 1.0),
            parchment_light: Vec4::new(0.940, 0.910, 0.800, 1.0),
            parchment_dark: Vec4::new(0.700, 0.650, 0.520, 1.0),
            foxing_color: Vec4::new(0.62, 0.55, 0.42, 1.0),
            stain_color: Vec4::new(0.76, 0.70, 0.58, 1.0),

            fiber_scale_h: Vec2::new(40.0, 8.0),
            mesh_ratio: Vec2::new(1.0, 1.0),
            fiber_scale_v: Vec2::new(8.0, 30.0),
            stain1_edge: Vec2::new(0.45, 0.65),
            stain2_edge: Vec2::new(0.35, 0.6),
            patch_edge: Vec2::new(0.35, 0.65),

            base_scale: 2.0,
            base_blend: 0.6,
            grain_scale: 18.0,
            grain_strength: 0.25,
            fiber_h_weight: 0.7,
            fiber_strength: 0.15,
            foxing_voronoi_scale: 20.0,
            foxing_noise_scale: 25.0,
            foxing_edge_low: 0.0,
            foxing_edge_high: 0.12,
            foxing_threshold: 0.5,
            foxing_strength: 0.6,
            stain1_scale: 4.0,
            stain2_scale: 3.0,
            stain_strength: 0.35,
            patch_scale: 1.5,
            patch_strength: 0.15,
            line_freq: 400.0,
            line_width: 0.35,
            line_var_scale_y: 4.0,
            line_shade_factor: 0.4,
            line_strength: 0.18,
            vignette_strength: 0.5,
            dither_scale: 1000.0,
            dither_strength: 0.01,

            base_octaves: 4,
            grain_octaves: 5,
            fiber_octaves_h: 4,
            fiber_octaves_v: 3,
            foxing_noise_octaves: 3,
            stain1_octaves: 4,
            stain2_octaves: 3,
            patch_octaves: 3,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ParchmentBgMaterial {
    #[uniform(0)]
    pub params: ParchmentTextureParams,
}

impl Material2d for ParchmentBgMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/parchment_bg.wgsl".into()
    }
}
