use std::f32::consts::PI;

use super::graticule::setup_circle_graticule;
use crate::config::MapConfig;
use crate::ecs::FontAssets;
use crate::layers::celestial::spawn_nine_heavens_diagram;
use crate::layers::graticule::setup_pseudocylindrical_graticule;
use crate::layers::overlays::celestial::spawn_armillary_sphere;
use crate::palette::ColorTheme;
use crate::projection::{kavrayskiy_vii, parallel_ratio};
use crate::render::graticule::indicator::GraticuleRingMaterial;
use crate::writing::WritingMode::VerticalRL;
use crate::writing::{spawn_vertical_text, VerticalTextLayout};
use bevy::prelude::*;

pub fn setup_overlays_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut graticule_ring_materials: ResMut<Assets<GraticuleRingMaterial>>,
    map_config: Res<MapConfig>,
    theme: Res<ColorTheme>,
    fonts: Res<FontAssets>,
) {
    // Main border
    commands.spawn((
        Mesh2d(
            meshes.add(
                Rectangle::new(map_config.canvas.size.x, map_config.canvas.size.y)
                    .to_ring(map_config.canvas.border_thickness),
            ),
        ),
        MeshMaterial2d(color_materials.add(theme.parchment.ink)),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, map_config.z.overlays)),
    ));

    {
        let pole_parallels: Vec<f32> = (40..=80).step_by(10).map(|n| n as f32).collect();

        // North Pole
        setup_circle_graticule(
            &mut commands,
            &mut meshes,
            &mut color_materials,
            &mut graticule_ring_materials,
            &map_config,
            Vec3::new(
                map_config.canvas.left()
                    + map_config.polar.radius
                    + map_config.polar.offset()
                    + map_config.canvas.margin.left,
                map_config.canvas.top()
                    - map_config.polar.radius
                    - map_config.polar.offset()
                    - map_config.canvas.margin.top,
                map_config.z.overlays,
            ),
            parallel_ratio,
            &pole_parallels,
            36,
            theme.parchment.ink,
        );

        // South Pole
        setup_circle_graticule(
            &mut commands,
            &mut meshes,
            &mut color_materials,
            &mut graticule_ring_materials,
            &map_config,
            Vec3::new(
                map_config.canvas.left()
                    + map_config.polar.radius
                    + map_config.polar.offset()
                    + map_config.canvas.margin.left,
                map_config.canvas.bottom()
                    + map_config.polar.radius
                    + map_config.polar.offset()
                    + map_config.canvas.margin.bottom,
                map_config.z.overlays,
            ),
            parallel_ratio,
            &pole_parallels,
            36,
            theme.parchment.ink,
        );
    }

    // Main map
    let ratio = (map_config.canvas.size.y - map_config.canvas.border_thickness * 2.0) / PI;
    let max_x = kavrayskiy_vii(Vec2::new(180.0, 0.0)).x
        * (ratio
            + map_config
                .main_map
                .borders_spacings
                .last()
                .unwrap_or(&0.0f32));

    setup_pseudocylindrical_graticule(
        &mut commands,
        &map_config,
        &mut meshes,
        &mut color_materials,
        Vec3::new(0., 0., 0.),
        ratio,
        (-180..=180).step_by(10).map(|n| n as f32).collect(),
        (-90..=90).step_by(10).map(|n| n as f32).collect(),
        theme.parchment.ink,
        kavrayskiy_vii,
    );

    {
        let line_thickness = map_config.note.lines.divider.value();
        let mut current_y = map_config.canvas.top();

        {
            let height = map_config.canvas.top() - map_config.canvas.bottom();
            let handler = meshes.add(Rectangle::new(line_thickness, height));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::new(
                    max_x + map_config.note.spacing.main,
                    0.0,
                    map_config.z.overlays,
                )),
            ));
        }

        // Horizontal line below title
        {
            let width = map_config.canvas.right() - (max_x + map_config.note.spacing.main);
            let y_pos = map_config.canvas.top() - map_config.note.title.font_size * 0.4;
            let handler = meshes.add(Rectangle::new(width, line_thickness));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::new(
                    max_x + map_config.note.spacing.main + width / 2.0,
                    y_pos,
                    map_config.z.overlays,
                )),
            ));
        }
        current_y -= map_config.note.title.font_size * 0.4 + line_thickness;

        // Map title
        let title_line_height = map_config.note.title.font_size * 1.3;
        let title_line_width = map_config.note.title.font_size * 1.1;
        let max_chars_per_column = 2;
        spawn_vertical_text(
            &mut commands,
            map_config.note.title.text,
            fonts.bold.clone(),
            Vec3::new(
                map_config.canvas.right() - map_config.note.title.font_size * 0.5,
                current_y,
                map_config.z.overlays,
            ),
            VerticalTextLayout {
                max_chars_per_column,
                char_spacing: title_line_height,
                column_spacing: title_line_width,
                mode: VerticalRL,
            },
            map_config.note.title.font_size,
            theme.parchment.ink,
        );
        current_y += -title_line_height * max_chars_per_column as f32;

        // Horizontal line below under-title section
        {
            let width = map_config.canvas.right() - (max_x + map_config.note.spacing.main);
            let handler = meshes.add(Rectangle::new(width, line_thickness));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::new(
                    max_x + map_config.note.spacing.main + width / 2.0,
                    current_y,
                    map_config.z.overlays,
                )),
            ));
        }
        current_y -= line_thickness;

        // Main note
        {
            let main_note_line_height = map_config.note.main_note.font_size * 1.15;
            let main_note_padding = map_config.note.main_note.font_size * 0.5;

            let main_note_max_chars_per_column = 137;

            let max_col = (map_config.note.main_note.text.chars().count() as f32
                / main_note_max_chars_per_column as f32)
                .ceil() as usize;
            let notes_width = map_config.canvas.right()
                - max_x
                - map_config.note.spacing.main
                - 2.0 * main_note_padding
                - line_thickness;
            let left_x = max_x + map_config.note.spacing.main + line_thickness;
            let main_note_line_width = (notes_width) / max_col as f32;

            spawn_vertical_text(
                &mut commands,
                map_config.note.main_note.text,
                fonts.bold.clone(),
                Vec3::new(
                    map_config.canvas.right() - main_note_padding + line_thickness,
                    current_y - map_config.note.main_note.font_size * 0.5, // margin top 0.5em
                    map_config.z.overlays,
                ),
                VerticalTextLayout {
                    max_chars_per_column: main_note_max_chars_per_column,
                    char_spacing: main_note_line_height,
                    column_spacing: main_note_line_width,
                    mode: VerticalRL,
                },
                map_config.note.main_note.font_size,
                theme.parchment.ink,
            );

            for i in 0..=max_col {
                // Add line_thickness ensure the line overlap other borders
                let height = current_y - map_config.canvas.bottom() + line_thickness;
                let handler = meshes.add(Rectangle::new(line_thickness, height));

                commands.spawn((
                    Mesh2d(handler),
                    MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                    Transform::default().with_translation(Vec3::new(
                        left_x + main_note_padding + (i as f32) * main_note_line_width,
                        // Move up mean draw from the separate line between border and the title
                        current_y - (height / 2.0) + line_thickness,
                        map_config.z.overlays,
                    )),
                ));
            }
        }

        // 九重天圖- Cửu Trùng Thiên Đồ - Diagram of the Nine Heavens.
        // Geocentric btw
        {
            let radius = map_config.polar.radius * 0.7;
            let position = Vec3::new(
                (max_x) - radius,
                map_config.canvas.top() - radius - map_config.note.spacing.main,
                map_config.z.overlays,
            );
            spawn_nine_heavens_diagram(
                &mut commands,
                &mut meshes,
                &mut color_materials,
                &mut graticule_ring_materials,
                &map_config,
                &theme,
                position,
                radius,
            );
        }
        // 天地儀- Thiên Địa Nghi - Heaven-and-Earth instrument
        // Armillary Sphere: celestial sphere projection diagram
        {
            let radius = map_config.polar.radius * 0.7;
            let position = Vec3::new(
                (max_x) - radius,
                map_config.canvas.bottom() + radius + map_config.note.spacing.main,
                map_config.z.overlays,
            );
            spawn_armillary_sphere(
                &mut commands,
                &mut meshes,
                &mut color_materials,
                &mut graticule_ring_materials,
                &map_config,
                &theme,
                position,
                radius,
            );
        }
    }
}
