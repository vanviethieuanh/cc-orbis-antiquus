use earcut::Earcut;
use std::f32::consts::PI;

use crate::{
    config::MapConfig,
    constants::{NORTH_SIGN, SOUTH_SIGN},
    ecs::MapData,
    palette::ColorTheme,
    projection::{azimuthal_equidistant_clipped, kavrayskiy_vii_ring, parallel_ratio},
};

use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};

pub fn setup_map_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_data: Res<MapData>,
    map_config: Res<MapConfig>,
    theme: Res<ColorTheme>,
) {
    {
        let scale = map_config.polar.radius / parallel_ratio(map_config.polar.lim_lat);
        let offset = map_config.polar.offset();

        // North pole
        draw_map(
            &mut commands,
            &mut meshes,
            &mut materials,
            &map_data,
            Vec3::new(
                map_config.canvas.left()
                    + map_config.polar.radius
                    + offset
                    + map_config.canvas.margin.left,
                map_config.canvas.top()
                    - map_config.polar.radius
                    - offset
                    - map_config.canvas.margin.top,
                map_config.z.map,
            ),
            |c| azimuthal_equidistant_clipped(c, NORTH_SIGN, NORTH_SIGN * map_config.polar.lim_lat),
            scale,
            theme.parchment.ink,
        );

        // South pole
        draw_map(
            &mut commands,
            &mut meshes,
            &mut materials,
            &map_data,
            Vec3::new(
                map_config.canvas.left()
                    + map_config.polar.radius
                    + offset
                    + map_config.canvas.margin.left,
                map_config.canvas.bottom()
                    + map_config.polar.radius
                    + offset
                    + map_config.canvas.margin.bottom,
                map_config.z.map,
            ),
            |c| azimuthal_equidistant_clipped(c, SOUTH_SIGN, SOUTH_SIGN * map_config.polar.lim_lat),
            scale,
            theme.parchment.ink,
        );
    }

    // Main map
    draw_map(
        &mut commands,
        &mut meshes,
        &mut materials,
        &map_data,
        Vec3::ZERO,
        kavrayskiy_vii_ring,
        (map_config.canvas.size.y - map_config.canvas.border_thickness * 2.0) / PI,
        theme.parchment.ink,
    );
}
pub fn draw_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    map_data: &MapData,
    position: Vec3,
    project_fn: impl Fn(&[Vec2]) -> Vec<Vec2>,
    ratio: f32,
    color: Color,
) {
    commands.spawn((
        Mesh2d(meshes.add(build_map_mesh(&map_data, project_fn, ratio))),
        MeshMaterial2d(materials.add(color)),
        Transform::default().with_translation(position),
    ));
}

pub fn build_map_mesh(
    map_data: &MapData,
    project_fn: impl Fn(&[Vec2]) -> Vec<Vec2>,
    ratio: f32,
) -> Mesh {
    let projected_positions: Vec<[f32; 3]> = map_data
        .polylines
        .iter()
        .flat_map(|ring| {
            let projected = project_fn(ring);

            let mut segments: Vec<[f32; 3]> = projected
                .windows(2)
                .flat_map(|w| {
                    let a = w[0] * ratio;
                    let b = w[1] * ratio;

                    [[a.x, a.y, 0.0], [b.x, b.y, 0.0]]
                })
                .collect();

            if let (Some(first), Some(last)) = (projected.first(), projected.last()) {
                let a = *last * ratio;
                let b = *first * ratio;

                segments.push([a.x, a.y, 0.0]);
                segments.push([b.x, b.y, 0.0]);
            }

            segments
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, projected_positions);
    mesh
}
