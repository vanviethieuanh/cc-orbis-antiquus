use bevy::prelude::*;
use crate::ecs::MapSettings;
use super::geospatial;

pub fn setup_map_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
) {
    if let Err(e) = geospatial::setup_map(commands, meshes, materials, settings, 1.0) {
        eprintln!("Error loading map: {}", e);
    }
}
