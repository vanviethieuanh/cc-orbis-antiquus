use super::components::CircleGraticuleGrid;
use super::graticule::setup_circle_graticule_grid;
use crate::constant::{
    CANVAS_BORDER_THICKNESS, CANVAS_BOT, CANVAS_LEFT, CANVAS_SIZE, CANVAS_TOP, OUTLINES_Z_INDEX,
    OVERLAYS_Z_INDEX, POLARS_RADIUS,
};
use crate::ecs::MapSettings;
use crate::layers::map::projections;
use crate::palette::PARCHMENT_INK;
use crate::render::indicator::GraticuleRingMaterial;
use crate::render::primitives::circle::CircleMaterial;
use bevy::prelude::*;

pub fn setup_overlays_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
    mut graticule_ring_materials: ResMut<Assets<GraticuleRingMaterial>>,
    settings: Res<MapSettings>,
) {
    let cli = &settings.cli;
    let d = cli.distance;
    let r_proj = projections::max_projected_radius(POLARS_RADIUS, d);

    setup_circle_graticule_grid(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &mut circle_materials,
        &mut graticule_ring_materials,
        &CircleGraticuleGrid::new(r_proj)
            .with_parallels(vec![40.0, 50.0, 60.0, 70.0, 80.0])
            .with_meridians(36),
        |lat: f32| projections::parallel_ratio(lat, POLARS_RADIUS, d),
        Vec3::new(
            CANVAS_LEFT + POLARS_RADIUS,
            CANVAS_TOP - POLARS_RADIUS,
            OVERLAYS_Z_INDEX,
        ),
    );

    setup_circle_graticule_grid(
        &mut commands,
        &mut meshes,
        &mut color_materials,
        &mut circle_materials,
        &mut graticule_ring_materials,
        &CircleGraticuleGrid::new(r_proj)
            .with_parallels(vec![40.0, 50.0, 60.0, 70.0, 80.0])
            .with_meridians(36),
        |lat: f32| projections::parallel_ratio(lat, POLARS_RADIUS, d),
        Vec3::new(
            CANVAS_LEFT + POLARS_RADIUS,
            CANVAS_BOT + POLARS_RADIUS,
            OVERLAYS_Z_INDEX,
        ),
    );

    let border =
        meshes.add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y).to_ring(CANVAS_BORDER_THICKNESS));
    commands.spawn((
        Mesh2d(border),
        MeshMaterial2d(color_materials.add(PARCHMENT_INK)),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, OUTLINES_Z_INDEX)),
    ));
}
