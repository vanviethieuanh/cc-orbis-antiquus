use bevy::{
    asset::Handle,
    color::Color,
    ecs::system::Commands,
    math::Vec3,
    sprite::Text2d,
    text::{Font, Justify, TextColor, TextFont, TextLayout},
    transform::components::Transform,
    utils::default,
};

pub enum WritingMode {
    VerticalLR,
    VerticalRL,
}

pub struct VerticalTextLayout {
    pub max_chars_per_column: usize,
    pub char_spacing: f32,
    pub column_spacing: f32,
    pub mode: WritingMode,
}

pub fn spawn_vertical_text(
    commands: &mut Commands,
    text: &str,
    font: Handle<Font>,
    base: Vec3,
    layout: VerticalTextLayout,
    font_size: f32,
    color: Color,
) {
    let chars: Vec<char> = text.chars().collect();
    let max_col = chars.len() / layout.max_chars_per_column;

    for (i, ch) in chars.iter().enumerate() {
        let col = i / layout.max_chars_per_column;
        let row = i % layout.max_chars_per_column;

        let x = match layout.mode {
            WritingMode::VerticalLR => col as f32 * layout.column_spacing,
            WritingMode::VerticalRL => ((max_col - col) as f32) * layout.column_spacing,
        } - (max_col as f32 + 0.5) * layout.column_spacing;

        let y = -(row as f32 + 0.5) * layout.char_spacing;

        commands.spawn((
            Text2d::new(ch.to_string()),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(base + Vec3::new(x, y, 0.0)),
        ));
    }
}
