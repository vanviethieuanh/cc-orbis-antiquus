use std::f32::consts::PI;

use super::graticule::setup_circle_graticule;
use crate::config::MapConfig;
use crate::layers::graticule::setup_pseudocylindrical_graticule;
use crate::palette::ColorTheme;
use crate::projection::{kavrayskiy_vii, parallel_ratio};
use crate::render::graticule::indicator::GraticuleRingMaterial;
use crate::render::primitives::circle::CircleMaterial;
use bevy::prelude::*;

pub fn setup_overlays_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
    mut graticule_ring_materials: ResMut<Assets<GraticuleRingMaterial>>,
    map_config: Res<MapConfig>,
    theme: Res<ColorTheme>,
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
            &mut circle_materials,
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
            &mut circle_materials,
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
    setup_pseudocylindrical_graticule(
        &mut commands,
        &map_config,
        &mut meshes,
        &mut color_materials,
        Vec3::new(0., 0., 0.),
        (map_config.canvas.size.y - map_config.canvas.border_thickness * 2.0) / PI,
        (-180..=180).step_by(10).map(|n| n as f32).collect(),
        (-90..=90).step_by(10).map(|n| n as f32).collect(),
        theme.parchment.ink,
        kavrayskiy_vii,
    )
}
