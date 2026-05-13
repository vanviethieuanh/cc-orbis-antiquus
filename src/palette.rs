use bevy::{color::Color, ecs::resource::Resource};

#[derive(Debug, Resource)]
pub struct ColorTheme {
    pub parchment: ParchmentColors,
}

#[derive(Debug, Resource)]
pub struct ParchmentColors {
    pub bg: Color,
    pub light: Color,
    pub medium: Color,
    pub dark: Color,
    pub ink: Color,
    pub accent: Color,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            parchment: ParchmentColors {
                bg: Color::srgb(0.920, 0.860, 0.640),
                light: Color::srgb(0.965, 0.935, 0.820),
                medium: Color::srgb(0.890, 0.800, 0.570),
                dark: Color::srgb(0.700, 0.620, 0.420),
                ink: Color::srgb(0.15, 0.10, 0.05),
                accent: Color::srgb(0.835, 0.682, 0.420),
            },
        }
    }
}
