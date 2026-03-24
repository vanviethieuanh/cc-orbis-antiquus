use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use shapefile::Reader;
use std::error::Error;

use crate::constant::POLARS_RADIUS;
use crate::ecs::{MapData, MapSettings};
use crate::palette::PARCHMENT_INK;
use crate::projection::perspective_pole;

pub fn draw_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    map: &MapData,
    position: Vec3,
    project_fn: impl Fn(f32, f32) -> (f32, f32),
    ratio: f32,
    color: Color,
) {
    commands.spawn((
        Mesh2d(meshes.add(build_map_mesh(&map, project_fn, ratio))),
        MeshMaterial2d(materials.add(color)),
        Transform::default().with_translation(position),
    ));
}

pub fn setup_pole(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &Res<MapSettings>,
    position: Vec3,
    project_center_deg: Vec2,
    points_filter_fn: impl Fn(f32, f32) -> bool,
) -> Result<(), Box<dyn Error>> {
    let cli = &settings.cli;
    let d = cli.distance;

    let mut reader = Reader::from_path(&cli.input)?;

    for result in reader.iter_shapes_and_records() {
        let (shape, _) = result?;

        if let shapefile::Shape::Polygon(poly) = shape {
            for ring in poly.rings() {
                let mut current_strip: Vec<Vec3> = Vec::new();

                for point in ring.points() {
                    let lon = point.x as f32;
                    let lat = point.y as f32;

                    if !points_filter_fn(lon, lat) {
                        continue;
                    }

                    let proj = perspective_pole(POLARS_RADIUS, lon, lat, project_center_deg.x, d);

                    if proj.visible {
                        let x = proj.x + position.x;
                        let y = proj.y + position.y;

                        current_strip.push(Vec3::new(x, y, 0.0));
                    } else {
                        spawn_line_strip(
                            commands,
                            meshes,
                            materials,
                            &current_strip,
                            PARCHMENT_INK,
                            position.z,
                        );
                        current_strip.clear();
                    }
                }

                spawn_line_strip(
                    commands,
                    meshes,
                    materials,
                    &current_strip,
                    PARCHMENT_INK,
                    position.z,
                );
            }
        }
    }

    Ok(())
}

fn spawn_line_strip(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions: &Vec<Vec3>,
    color: Color,
    z_index: f32,
) {
    if positions.len() < 2 {
        return;
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, z_index)),
    ));
}

pub fn build_map_mesh(
    map: &MapData,
    project_fn: impl Fn(f32, f32) -> (f32, f32),
    ratio: f32,
) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();

    for ring in &map.polylines {
        for window in ring.windows(2) {
            let a = window[0];
            let b = window[1];

            let (x1, y1) = project_fn(a.x, a.y);
            let (x2, y2) = project_fn(b.x, b.y);

            positions.push([x1 * ratio, y1 * ratio, 0.0]);
            positions.push([x2 * ratio, y2 * ratio, 0.0]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    mesh
}
