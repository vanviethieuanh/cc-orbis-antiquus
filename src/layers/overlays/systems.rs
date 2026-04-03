use std::f32::consts::PI;

use super::components::CircleGraticuleGrid;
use super::graticule::setup_circle_graticule;
use crate::constant::{
    CANVAS_BORDER_THICKNESS, CANVAS_BOT, CANVAS_LEFT, CANVAS_MARGIN, CANVAS_SIZE, CANVAS_TOP,
    OVERLAYS_Z_INDEX, POLARS_RADIUS, POLE_VIEWPOINT_DISTANCE,
};

use crate::layers::graticule::setup_pseudocylindrical_graticule;
use crate::palette::PARCHMENT_INK;
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
) {
    // Main border
    commands.spawn((
        Mesh2d(
            meshes
                .add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y).to_ring(CANVAS_BORDER_THICKNESS)),
        ),
        MeshMaterial2d(color_materials.add(PARCHMENT_INK)),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, OVERLAYS_Z_INDEX)),
    ));

    // North Pole
    let north_grid = &CircleGraticuleGrid::new(POLARS_RADIUS)
        .with_parallels(vec![40.0, 50.0, 60.0, 70.0, 80.0])
        .with_meridians(36);
    setup_circle_graticule(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &mut circle_materials,
        &mut graticule_ring_materials,
        north_grid,
        |lat: f32| parallel_ratio(lat, POLARS_RADIUS, POLE_VIEWPOINT_DISTANCE),
        Vec3::new(
            CANVAS_LEFT
                + POLARS_RADIUS
                + north_grid.graticule_ring_thickness * 3.0
                + CANVAS_MARGIN.3,
            CANVAS_TOP
                - POLARS_RADIUS
                - north_grid.graticule_ring_thickness * 3.0
                - CANVAS_MARGIN.0,
            OVERLAYS_Z_INDEX,
        ),
    );

    // South Pole
    let south_grid = &CircleGraticuleGrid::new(POLARS_RADIUS)
        .with_parallels(vec![40.0, 50.0, 60.0, 70.0, 80.0])
        .with_meridians(36);
    setup_circle_graticule(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &mut circle_materials,
        &mut graticule_ring_materials,
        south_grid,
        |lat: f32| parallel_ratio(lat, POLARS_RADIUS, POLE_VIEWPOINT_DISTANCE),
        Vec3::new(
            CANVAS_LEFT
                + POLARS_RADIUS
                + south_grid.graticule_ring_thickness * 3.0
                + CANVAS_MARGIN.3,
            CANVAS_BOT
                + POLARS_RADIUS
                + south_grid.graticule_ring_thickness * 3.0
                + CANVAS_MARGIN.2,
            OVERLAYS_Z_INDEX,
        ),
    );

    // Main map
    setup_pseudocylindrical_graticule(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        Vec3::new(0., 0., 0.),
        (CANVAS_SIZE.y - CANVAS_BORDER_THICKNESS * 2.0) / (PI),
        (-180..=180).step_by(10).map(|n| n as f32).collect(),
        (-90..=90).step_by(10).map(|n| n as f32).collect(),
        PARCHMENT_INK,
        kavrayskiy_vii,
    )
}
