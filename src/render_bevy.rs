use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use shapefile::Reader;
use std::error::Error;

use crate::cli::Cli;
use crate::projections::{self};

const COASTLINE_COLOR: Color = Color::BLACK;

#[derive(Resource)]
pub struct MapSettings {
    pub cli: Cli,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
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
                );
            }
        }
    }

    setup_degree_ring(&mut commands, meshes, materials, r_earth, d)?;

    Ok(())
}

fn setup_degree_ring(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    r_earth: f32,
    d: f32,
) -> Result<(), Box<dyn Error>> {
    let r_proj = projections::max_projected_radius(r_earth, d);
    let thickness = 16.0;

    let outer_radius = (r_proj + 4.0 * thickness) as f32;
    let circle_segments = 128;

    let mut circle_positions = Vec::new();
    for i in 0..=circle_segments {
        let angle = (i as f32 / circle_segments as f32) * std::f32::consts::TAU;
        let x = outer_radius * angle.cos();
        let y = outer_radius * angle.sin();
        circle_positions.push(Vec3::new(x, y, 0.0));
    }

    if !circle_positions.is_empty() {
        let mut circle_mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        );
        circle_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, circle_positions);

        let mesh_handle = meshes.add(circle_mesh);
        let material_handle = materials.add(ColorMaterial::from_color(Color::BLACK));

        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::default(),
        ));
    }

    for i in (5..9).rev() {
        let parallel_r =
            (projections::parallel_ratio(i as f32 * 10.0, r_earth, d) * r_earth) as f32;

        let mut parallel_positions = Vec::new();
        for j in 0..=circle_segments {
            let angle = (j as f32 / circle_segments as f32) * std::f32::consts::TAU;
            let x = parallel_r * angle.cos();
            let y = parallel_r * angle.sin();
            parallel_positions.push(Vec3::new(x, y, 0.0));
        }

        if !parallel_positions.is_empty() {
            let mut parallel_mesh = Mesh::new(
                PrimitiveTopology::LineStrip,
                RenderAssetUsages::RENDER_WORLD,
            );
            parallel_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, parallel_positions);

            let mesh_handle = meshes.add(parallel_mesh);
            let material_handle = materials.add(ColorMaterial::from_color(Color::srgb(
                120.0 / 255.0,
                120.0 / 255.0,
                120.0 / 255.0,
            )));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::default(),
            ));
        }
    }

    let sections = 36;
    for i in 0..sections {
        let angle = (i as f32 / sections as f32) * std::f32::consts::TAU;
        let parallel_80th_r = (projections::parallel_ratio(80.0, r_earth, d) * r_earth) as f32;

        let x0 = parallel_80th_r * angle.sin();
        let y0 = -parallel_80th_r * angle.cos();
        let x1 = (r_proj + 4.0 * thickness) * angle.sin();
        let y1 = -(r_proj + 4.0 * thickness) * angle.cos();

        let positions = vec![Vec3::new(x0, y0, 0.0), Vec3::new(x1, y1, 0.0)];

        let mut line_mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        );
        line_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        let mesh_handle = meshes.add(line_mesh);
        let material_handle = materials.add(ColorMaterial::from_color(Color::srgb(
            120.0 / 255.0,
            120.0 / 255.0,
            120.0 / 255.0,
        )));

        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::default(),
        ));
    }

    Ok(())
}

fn spawn_line_strip(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions: &Vec<Vec3>,
    color: Color,
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
        Transform::default(),
    ));
}
