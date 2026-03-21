use super::components::CircleGraticuleGrid;
use super::graticule::setup_circle_graticule_grid;
use crate::constant::{CANVAS_LEFT, CANVAS_TOP, OVERLAYS_Z_INDEX, POLARS_RADIUS};
use crate::ecs::MapSettings;
use crate::layers::map::projections;
use crate::render::indicator::GraticuleRingMaterial;
use crate::render::primitives::circle::CircleMaterial;
use bevy::prelude::*;

pub fn setup_overlays_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
    circle_materials: ResMut<Assets<CircleMaterial>>,
    graticule_ring_materials: ResMut<Assets<GraticuleRingMaterial>>,
    settings: Res<MapSettings>,
) {
    let cli = &settings.cli;
    let d = cli.distance;
    let r_proj = projections::max_projected_radius(POLARS_RADIUS, d);

    // Create circle graticule grid for the main world map projection
    // Setup graticule - pass closure that captures projection parameters
    setup_circle_graticule_grid(
        commands,
        meshes,
        color_materials,
        circle_materials,
        graticule_ring_materials,
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
}
