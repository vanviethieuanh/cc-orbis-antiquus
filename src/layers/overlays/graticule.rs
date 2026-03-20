use crate::render::primitives::circle::{spawn_circle, CircleMaterial};

use super::components::CircleGraticuleGrid;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

pub fn setup_circle_graticule_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
    grid: &CircleGraticuleGrid,
    parallel_ratio_fn: impl Fn(f32) -> f32,
    z_index: f32,
) {
    // Meridians
    {
        let start_radius = if let Some(&highest_latitude) = (&grid.parallels).last() {
            (&parallel_ratio_fn)(highest_latitude) * grid.radius
        } else {
            0.0
        };

        for i in 0..grid.meridians {
            let angle = (i as f32 / grid.meridians as f32) * std::f32::consts::TAU;
            let x_outer = grid.radius * angle.sin();
            let y_outer = -grid.radius * angle.cos();
            let x_inner = start_radius * angle.sin();
            let y_inner = -start_radius * angle.cos();

            let positions = vec![
                Vec3::new(x_inner, y_inner, 0.0),
                Vec3::new(x_outer, y_outer, 0.0),
            ];

            let mut line_mesh = Mesh::new(
                PrimitiveTopology::LineStrip,
                RenderAssetUsages::RENDER_WORLD,
            );
            line_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

            let mesh_handle = (&mut meshes).add(line_mesh);
            let material_handle =
                (&mut color_materials).add(ColorMaterial::from_color(grid.meridian_color));

            (&mut commands).spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::default().with_translation(Vec3::new(0.0, 0.0, z_index)),
            ));
        }
    };

    // Parallels (concentric circles)
    {
        let outer_radius: f32 = grid.radius;
        for &latitude in &grid.parallels {
            spawn_circle(
                &mut commands,
                &mut meshes,
                &mut circle_materials,
                Vec3::new(0.0, 0.0, z_index),
                parallel_ratio_fn(latitude) * outer_radius * 2.0,
                1.0,
                0.5,
                Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            );
        }
    };

    // Draw outer boundary circle
    spawn_circle(
        &mut commands,
        &mut meshes,
        &mut circle_materials,
        Vec3::new(0.0, 0.0, z_index),
        2.0 * grid.radius,
        2.0,
        0.5,
        Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
        Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
    );
}
