use crate::constant::MAIN_MAP_MEDIAN_SEGMENTS;
use crate::render::graticule::indicator::{spawn_graticule_ring, GraticuleRingMaterial};
use crate::render::primitives::circle::{spawn_circle, CircleMaterial};

use super::components::CircleGraticuleGrid;
use bevy::prelude::*;

pub fn setup_circle_graticule(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    circle_materials: &mut ResMut<Assets<CircleMaterial>>,
    graticule_ring_materials: &mut ResMut<Assets<GraticuleRingMaterial>>,
    grid: &CircleGraticuleGrid,
    parallel_ratio_fn: impl Fn(f32) -> f32,
    position: Vec3,
) {
    // Meridians
    {
        let start_radius = if let Some(&highest_latitude) = (&grid.parallels).last() {
            (&parallel_ratio_fn)(highest_latitude) * grid.radius
        } else {
            0.0
        };

        let end_radius = grid.radius + 3. * grid.graticule_ring_thickness;

        for i in 0..grid.meridians {
            let angle = (i as f32 / grid.meridians as f32) * std::f32::consts::TAU;
            let x_outer = end_radius * angle.sin();
            let y_outer = -end_radius * angle.cos();
            let x_inner = start_radius * angle.sin();
            let y_inner = -start_radius * angle.cos();

            let mesh_handle = (meshes).add(Segment2d::new(
                Vec2::new(x_inner, y_inner),
                Vec2::new(x_outer, y_outer),
            ));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(color_materials.add(grid.meridian_color)),
                Transform::default().with_translation(position),
            ));
        }
    };

    // Parallels
    {
        let outer_radius: f32 = grid.radius;
        for &latitude in &grid.parallels {
            spawn_circle(
                commands,
                meshes,
                circle_materials,
                position,
                parallel_ratio_fn(latitude) * outer_radius * 2.0,
                1.0,
                0.5,
                grid.parallel_color.into(),
                Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            );
        }
    };

    // First boundary circle
    spawn_circle(
        commands,
        meshes,
        circle_materials,
        position,
        2.0 * grid.radius,
        grid.boundary_thickness,
        0.5,
        grid.boundary_color.into(),
        Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
    );

    // spawn_graticule_ring
    spawn_graticule_ring(
        commands,
        meshes,
        graticule_ring_materials,
        position,
        // 0.9 make the right slightly tigher to inner border, avoid render leak create a
        //   slight white ring.
        2.0 * (grid.radius + grid.graticule_ring_thickness * 0.9),
        360.,
        grid.graticule_ring_thickness,
        0.5,
        0.025,
        grid.boundary_color.into(),
    );

    // Second boundary circle
    spawn_circle(
        commands,
        meshes,
        circle_materials,
        position,
        2.0 * (grid.radius + grid.graticule_ring_thickness),
        grid.boundary_thickness,
        0.5,
        grid.boundary_color.into(),
        Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
    );

    // Third boundary circle
    spawn_circle(
        commands,
        meshes,
        circle_materials,
        position,
        2.0 * (grid.radius + 3. * grid.graticule_ring_thickness),
        grid.boundary_thickness,
        0.5,
        grid.boundary_color.into(),
        Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
    );
}

pub fn setup_pseudocylindrical_graticule(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    ratio: f32,
    meridians: Vec<f32>,
    parallels: Vec<f32>,
    color: Color,
    projection_fn: impl Fn(Vec2) -> Vec2,
) {
    // Meridians
    {
        for long in meridians {
            let med = (meshes).add(Polyline2d::new(
                (0..(MAIN_MAP_MEDIAN_SEGMENTS + 1))
                    .map(|i| -90. + i as f32 * (180.0 / MAIN_MAP_MEDIAN_SEGMENTS as f32))
                    .map(|lat| projection_fn(Vec2::new(long, lat)) * ratio),
            ));

            commands.spawn((
                Mesh2d(med),
                MeshMaterial2d(color_materials.add(color)),
                Transform::default().with_translation(position),
            ));
        }
    };

    // Parallels
    {
        for lat in parallels {
            let med = (meshes).add(Segment2d::new(
                projection_fn(Vec2::new(-180.0, lat)) * ratio,
                projection_fn(Vec2::new(180.0, lat)) * ratio,
            ));

            commands.spawn((
                Mesh2d(med),
                MeshMaterial2d(color_materials.add(color)),
                Transform::default().with_translation(position),
            ));
        }
    };

    // Indicators
    {};

    // Notes
    {};
}
