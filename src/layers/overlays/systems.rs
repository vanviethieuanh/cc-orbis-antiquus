use bevy::prelude::*;
use crate::ecs::MapSettings;
use super::components::CircleGraticuleGrid;
use super::graticule::setup_circle_graticule_grid;
use crate::layers::map::projections;

pub fn setup_overlays_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
) {
    let cli = &settings.cli;
    let r_earth = cli.radius;
    let d = cli.distance;
    let r_proj = projections::max_projected_radius(r_earth, d);

    // Create circle graticule grid for the main world map projection
    let grid = CircleGraticuleGrid::new(r_proj)
        .with_parallels(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0])
        .with_meridians(36);

    // Setup graticule - pass closure that captures projection parameters
    setup_circle_graticule_grid(
        commands,
        meshes,
        materials,
        &grid,
        |lat: f32| projections::parallel_ratio(lat, r_earth, d),
        2.0, // z-index for overlays layer
    );

    // Phase 1: Graticule grid is now set up
    // TODO Phase 2: Add text labels, decorative elements
    // TODO Phase 3: Add animation support for overlays
}
