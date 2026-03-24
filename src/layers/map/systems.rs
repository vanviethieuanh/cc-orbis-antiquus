use earcut::Earcut;
use std::f32::consts::PI;

use crate::{
    constant::{
        CANVAS_BORDER_THICKNESS, CANVAS_BOT, CANVAS_LEFT, CANVAS_SIZE, CANVAS_TOP, MAP_Z_INDEX,
        POLARS_RADIUS, POLE_VIEWPOINT_DISTANCE,
    },
    ecs::MapData,
    palette::PARCHMENT_INK,
    projection::{kavrayskiy_vii, perspective_polar_projection_clamped},
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub fn setup_map_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<MapData>,
) {
    // North pole
    draw_map(
        &mut commands,
        &mut meshes,
        &mut materials,
        &map,
        |_long, lat| lat.abs() != 90.0,
        Vec3::new(CANVAS_LEFT, CANVAS_TOP - POLARS_RADIUS * 2., MAP_Z_INDEX),
        |c| perspective_polar_projection_clamped(1.0, c, 0.0, POLE_VIEWPOINT_DISTANCE, 1.0),
        POLARS_RADIUS,
        PARCHMENT_INK,
    );

    // South pole
    draw_map(
        &mut commands,
        &mut meshes,
        &mut materials,
        &map,
        |_long, lat| lat.abs() != 90.0,
        Vec3::new(CANVAS_LEFT, CANVAS_BOT, MAP_Z_INDEX),
        |c| perspective_polar_projection_clamped(1.0, c, 0.0, POLE_VIEWPOINT_DISTANCE, -1.0),
        POLARS_RADIUS,
        PARCHMENT_INK,
    );

    // Main map
    draw_map(
        &mut commands,
        &mut meshes,
        &mut materials,
        &map,
        |_long, _lat| true,
        Vec3::ZERO,
        kavrayskiy_vii,
        (CANVAS_SIZE.y - CANVAS_BORDER_THICKNESS * 2.0) / (PI),
        PARCHMENT_INK,
    );
}

pub fn draw_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    map: &MapData,
    points_filter_fn: impl Fn(f32, f32) -> bool,
    position: Vec3,
    project_fn: impl Fn(Vec2) -> Vec2,
    ratio: f32,
    color: Color,
) {
    commands.spawn((
        Mesh2d(meshes.add(build_map_mesh(&map, points_filter_fn, project_fn, ratio))),
        MeshMaterial2d(materials.add(color)),
        Transform::default().with_translation(position),
    ));
}

pub fn build_map_mesh(
    map: &MapData,
    points_filter_fn: impl Fn(f32, f32) -> bool,
    project_fn: impl Fn(Vec2) -> (Vec2),
    ratio: f32,
) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let eps = 1e-6;

    for ring in &map.polylines {
        for window in ring.windows(2) {
            let a = window[0];
            let b = window[1];

            if !points_filter_fn(a.x, a.y) || !points_filter_fn(b.x, b.y) {
                continue;
            }

            let pa = project_fn(a) * ratio;
            let pb = project_fn(b) * ratio;

            if pa.distance_squared(pb) < eps {
                continue;
            }

            positions.push([pa.x, pa.y, 0.0]);
            positions.push([pb.x, pb.y, 0.0]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh
}
