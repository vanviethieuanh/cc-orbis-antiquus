use std::{f32::consts::PI, fmt};

use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    config::MapConfig,
    ecs::FontAssets,
    palette::ColorTheme,
    render::graticule::{spawn_graticule_ring, GraticuleRingMaterial},
    utils::draw_ring,
};

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, PartialOrd)]
enum CelestialSphere {
    Moon,
    Mercury,
    Venus,
    Sun,
    Mars,
    Jupiter,
    Saturn,
    FixedStars,
    PrimumMobile,
}

impl fmt::Display for CelestialSphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialSphere::Moon => "第一重月輪天二十七日三十一刻作一周自西而東",
                CelestialSphere::Mercury => "第二重辰星即水星天三百六十五日二十三刻作一周自西而東",
                CelestialSphere::Venus => "第三重太白即金星天三百六十五日二十三刻作一周自西而東",
                CelestialSphere::Sun => "第四重日輪天三百六十五日二十三刻作一周自西而東",
                CelestialSphere::Mars => "第五重螢惑即火星天一年三百二十一日九十三刻作一周自西而東",
                CelestialSphere::Jupiter =>
                    "第六重歲星即木星天十一年三百一十三日七十刻作一周自西而東",
                CelestialSphere::Saturn =>
                    "第七重填星即土星天二十九年一百五十五日二十五刻作一周自西而東",
                CelestialSphere::FixedStars => "第八重天二十八宿天七千年作一周由西而東",
                CelestialSphere::PrimumMobile => "第九重天無星帶八重天轉動一日作一周自東而西",
            }
        )
    }
}

pub(super) fn spawn_armillary_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    graticule_ring_materials: &mut ResMut<Assets<GraticuleRingMaterial>>,
    map_config: &Res<MapConfig>,
    theme: &Res<ColorTheme>,
    position: Vec3,
    radius: f32,
) {
    draw_ring(
        commands,
        meshes,
        color_materials,
        radius - 3.0 * map_config.polar.ring_thickness,
        map_config.polar.stroke_thickness,
        position,
        theme.parchment.ink.into(),
    );

    draw_ring(
        commands,
        meshes,
        color_materials,
        radius - map_config.polar.ring_thickness,
        map_config.polar.stroke_thickness,
        position,
        theme.parchment.ink.into(),
    );

    spawn_graticule_ring(
        commands,
        meshes,
        graticule_ring_materials,
        position,
        // 0.9 make the right slightly tigher to inner border, avoid render leak create a
        //   slight white ring.
        2.0 * radius,
        360.,
        map_config.polar.ring_thickness,
        0.5,
        0.025,
        theme.parchment.ink.into(),
    );

    draw_ring(
        commands,
        meshes,
        color_materials,
        radius,
        map_config.polar.stroke_thickness,
        position,
        theme.parchment.ink.into(),
    );
}

pub(super) fn spawn_nine_heavens_diagram(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    graticule_ring_materials: &mut ResMut<Assets<GraticuleRingMaterial>>,
    map_config: &Res<MapConfig>,
    theme: &Res<ColorTheme>,
    position: Vec3,
    radius: f32,
    fonts: &Res<FontAssets>,
) {
    let font_size = 20.0;
    let ring_gap = font_size + 8.0;

    let stroke_thickness = 1.0;

    let material = color_materials.add(theme.parchment.ink);

    for (i, planet) in CelestialSphere::iter().rev().enumerate() {
        let r = radius - i as f32 * ring_gap;

        let inner = r - stroke_thickness * 0.5;
        let outer = r + stroke_thickness * 0.5;

        let mesh_handle = meshes.add(
            Annulus::new(inner.max(0.0), outer)
                .mesh()
                .resolution(64)
                .build(),
        );

        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(position),
        ));

        // ---------------------------------------------------------
        // Curved text between rings
        // ---------------------------------------------------------

        let label = planet.to_string();

        let text_radius = r - ring_gap * 0.5;
        let glyph_width = font_size * 1.0;
        let circumference = std::f32::consts::TAU * text_radius;
        let angle_step = (glyph_width / circumference) * std::f32::consts::TAU;

        // center text around top
        let start_angle = std::f32::consts::FRAC_PI_2;

        for (ch_index, ch) in label.chars().enumerate() {
            let theta = if planet == CelestialSphere::PrimumMobile {
                start_angle + (ch_index as f32 + 0.5) * angle_step
            } else {
                start_angle - (ch_index as f32 + 0.5) * angle_step
            };
            let rotation = if planet == CelestialSphere::PrimumMobile {
                Quat::from_rotation_z(theta + PI)
            } else {
                Quat::from_rotation_z(theta)
            };

            let x = theta.cos() * text_radius;
            let y = theta.sin() * text_radius;

            commands.spawn((
                Text2d::new(ch.to_string()),
                TextFont {
                    font: fonts.bold.clone(),
                    font_size,
                    ..default()
                },
                TextColor(theme.parchment.ink),
                Transform {
                    translation: position + Vec3::new(x, y, 1.0),
                    rotation: rotation,
                    ..default()
                },
            ));
        }
    }
}
