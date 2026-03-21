use super::geospatial;
use crate::{
    constant::{CANVAS_LEFT, CANVAS_TOP, MAP_Z_INDEX, POLARS_RADIUS},
    ecs::MapSettings,
};
use bevy::prelude::*;

pub fn setup_map_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
) {
    if let Err(e) = geospatial::setup_map(
        commands,
        meshes,
        materials,
        settings,
        Vec3::new(CANVAS_LEFT, CANVAS_TOP - 2.0 * POLARS_RADIUS, MAP_Z_INDEX),
    ) {
        eprintln!("Error loading map: {}", e);
    }
}
