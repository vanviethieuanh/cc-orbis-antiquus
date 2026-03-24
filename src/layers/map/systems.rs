use std::f32::consts::PI;

use super::geospatial;
use crate::{
    constant::{
        CANVAS_BORDER_THICKNESS, CANVAS_BOT, CANVAS_LEFT, CANVAS_SIZE, CANVAS_TOP, MAP_Z_INDEX,
        POLARS_RADIUS,
    },
    ecs::{MapData, MapSettings},
    layers::geospatial::draw_map,
    palette::PARCHMENT_INK,
    projection::kavrayskiy_vii,
};
use bevy::{math::VectorSpace, prelude::*};

pub fn setup_map_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<MapData>,
    settings: Res<MapSettings>,
) {
    // North pole
    if let Err(e) = geospatial::setup_pole(
        &mut commands,
        &mut meshes,
        &mut materials,
        &settings,
        Vec3::new(CANVAS_LEFT, CANVAS_TOP - 2.0 * POLARS_RADIUS, MAP_Z_INDEX),
        Vec2::new(0.0, 90.0),
        |_long, lat| lat > 0.0 && lat.abs() != 90.,
    ) {
        eprintln!("Error loading map: {}", e);
    }

    // South pole
    if let Err(e) = geospatial::setup_pole(
        &mut commands,
        &mut meshes,
        &mut materials,
        &settings,
        Vec3::new(CANVAS_LEFT, CANVAS_BOT, MAP_Z_INDEX),
        Vec2::new(0.0, -90.0),
        |_long, lat| lat < 0.0 && lat.abs() != 90.,
    ) {
        eprintln!("Error loading map: {}", e);
    }

    draw_map(
        &mut commands,
        &mut meshes,
        &mut materials,
        &map,
        Vec3::ZERO,
        kavrayskiy_vii,
        (CANVAS_SIZE.y - CANVAS_BORDER_THICKNESS * 2.0) / (PI),
        PARCHMENT_INK,
    );
}
