use std::fmt;

use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res, ResMut},
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use bevy::prelude::*;

use crate::{
    config::MapConfig,
    palette::ColorTheme,
    render::graticule::{spawn_graticule_ring, GraticuleRingMaterial},
    utils::draw_ring,
};

#[derive(Debug, Clone, Copy)]
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

impl CelestialSphere {
    /// Returns the exact Chinese string for the layer as written in the 九重天圖
    fn to_chinese_text(&self) -> &str {
        match self {
            CelestialSphere::Moon => "第一重月輪天",
            CelestialSphere::Mercury => "第二重辰星天", // Historically Mercury is 辰星
            CelestialSphere::Venus => "第三重太白天",   // Historically Venus is 太白
            CelestialSphere::Sun => "第四重日輪天",
            CelestialSphere::Mars => "第五重熒惑天", // Historically Mars is 熒惑
            CelestialSphere::Jupiter => "第六重歲星天", // Historically Jupiter is 歲星
            CelestialSphere::Saturn => "第七重填星天", // Historically Saturn is 填星
            CelestialSphere::FixedStars => "第八重二十八宿天",
            CelestialSphere::PrimumMobile => "第九重宗動天",
        }
    }

    /// Returns the orbital period details using Chinese characters found in the map's annotations
    fn orbital_period_traditional(&self) -> &str {
        match self {
            CelestialSphere::Moon => "二十七日三十一刻",
            CelestialSphere::Mercury => "三百六十五日二十三刻",
            CelestialSphere::Venus => "三百六十五日二十三刻",
            CelestialSphere::Sun => "三百六十五日二十三刻",
            CelestialSphere::Mars => "一年三百二十一日",
            CelestialSphere::Jupiter => "十一年三百一十三刻",
            CelestialSphere::Saturn => "二十九年一百五十五日",
            CelestialSphere::FixedStars => "自西而東", // "From West to East" movement
            CelestialSphere::PrimumMobile => "一日自東而西一周", // "One rotation East to West per day"
        }
    }
}

impl fmt::Display for CelestialSphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_chinese_text())
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
) {
    let stroke_thickness = 10.0;

    let mesh_handle = (meshes).add(
        Annulus::new(radius, radius + stroke_thickness)
            .mesh()
            .resolution(64)
            .build(),
    );
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(color_materials.add(theme.parchment.ink)),
        Transform::default().with_translation(position),
    ));
}
