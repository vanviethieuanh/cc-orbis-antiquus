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
                bg: Color::srgb(0.902, 0.827, 0.663),
                light: Color::srgb(0.960, 0.920, 0.867),
                medium: Color::srgb(0.878, 0.765, 0.596),
                dark: Color::srgb(0.686, 0.584, 0.447),
                ink: Color::srgb(0.15, 0.10, 0.05),
                accent: Color::srgb(0.835, 0.682, 0.420),
            },
        }
    }
}
