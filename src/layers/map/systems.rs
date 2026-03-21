use super::geospatial;
use crate::{constant::MAP_Z_INDEX, ecs::MapSettings};
use bevy::prelude::*;

pub fn setup_map_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
) {
    if let Err(e) = geospatial::setup_map(commands, meshes, materials, settings, MAP_Z_INDEX) {
        eprintln!("Error loading map: {}", e);
    }
}
