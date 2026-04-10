use std::f32::consts::PI;

use super::graticule::setup_circle_graticule;
use crate::config::MapConfig;
use crate::ecs::FontAssets;
use crate::layers::graticule::setup_pseudocylindrical_graticule;
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
        {
            let handler = (meshes).add(Segment2d::new(
                Vec2::new(
                    max_x + map_config.note.spacing.main,
                    map_config.canvas.top(),
                ),
                Vec2::new(
                    max_x + map_config.note.spacing.main,
                    map_config.canvas.bottom(),
                ),
            ));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::ZERO),
            ));
        }

        {
            let handler = (meshes).add(Segment2d::new(
                Vec2::new(
                    max_x + map_config.note.spacing.main,
                    map_config.canvas.top() - map_config.note.title.font_size * 0.4,
                ),
                Vec2::new(
                    map_config.canvas.right(),
                    map_config.canvas.top() - map_config.note.title.font_size * 0.4,
                ),
            ));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::ZERO),
            ));
        }

        {
            let handler = (meshes).add(Segment2d::new(
                Vec2::new(
                    max_x + map_config.note.spacing.main,
                    map_config.canvas.top() - map_config.note.title.font_size * ((0.4 + 1.3) * 2.0),
                ),
                Vec2::new(
                    map_config.canvas.right(),
                    map_config.canvas.top() - map_config.note.title.font_size * ((0.4 + 1.3) * 2.0),
                ),
            ));
            commands.spawn((
                Mesh2d(handler),
                MeshMaterial2d(color_materials.add(theme.parchment.ink)),
                Transform::default().with_translation(Vec3::ZERO),
            ));
        }

        spawn_vertical_text(
            &mut commands,
            map_config.note.title.text,
            fonts.bold.clone(),
            Vec3::new(
                map_config.canvas.right() - map_config.note.title.font_size * 0.5,
                map_config.canvas.top() - map_config.note.title.font_size * 0.5,
                map_config.z.overlays,
            ),
            VerticalTextLayout {
                max_chars_per_column: 2,
                char_spacing: map_config.note.title.font_size * 1.3,
                column_spacing: map_config.note.title.font_size * 1.1,
                mode: VerticalRL,
            },
            map_config.note.title.font_size,
            theme.parchment.ink,
        );

        spawn_vertical_text(
            &mut commands,
            map_config.note.main_note.text,
            fonts.bold.clone(),
            Vec3::new(
                map_config.canvas.right() - map_config.note.main_note.font_size * 0.5,
                map_config.canvas.top()
                    - map_config.note.title.font_size * ((0.4 + 1.3) * 2.0)
                    - map_config.note.main_note.font_size * 0.5,
                map_config.z.overlays,
            ),
            VerticalTextLayout {
                max_chars_per_column: 136,
                char_spacing: map_config.note.main_note.font_size * 1.15,
                column_spacing: map_config.note.main_note.font_size * 1.7,
                mode: VerticalRL,
            },
            map_config.note.main_note.font_size,
            theme.parchment.ink,
        );
    }
}

