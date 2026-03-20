use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use shapefile::Reader;
use std::error::Error;

use super::projections;
use crate::cli::Cli;
use crate::ecs::MapSettings;

const COASTLINE_COLOR: Color = Color::BLACK;

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
    z_index: f32,
) -> Result<(), Box<dyn Error>> {
    let cli = &settings.cli;
    let r_earth = cli.radius;
    let margin = cli.margin;
    let d = cli.distance;
    let (window_width, window_height) = cli.compute_window_size();

    let mut reader = Reader::from_path(&cli.input)?;

    for result in reader.iter_shapes_and_records() {
        let (shape, _) = result?;

        if let shapefile::Shape::Polygon(poly) = shape {
            for ring in poly.rings() {
                let mut current_strip: Vec<Vec3> = Vec::new();

                for point in ring.points() {
                    let lon = point.x as f32;
                    let lat = point.y as f32;

                    if cli.north_only && lat < 0.0 {
                        continue;
                    }

                    let proj = projections::perspective_pole(r_earth, lon, lat, d);

                    if proj.visible {
                        let x = proj.x + margin - window_width as f32 / 2.0;
                        let y = window_height as f32 / 2.0 - (proj.y + margin);

                        current_strip.push(Vec3::new(x, y, 0.0));
                    } else {
                        spawn_line_strip(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            &current_strip,
                            COASTLINE_COLOR,
                            z_index,
                        );
                        current_strip.clear();
                    }
                }

                spawn_line_strip(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &current_strip,
                    COASTLINE_COLOR,
                    z_index,
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
