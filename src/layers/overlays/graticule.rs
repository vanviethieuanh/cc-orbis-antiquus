use crate::render::graticule::KavrayskiyViiGraticuleMaterial;
use crate::render::indicator::{spawn_graticule_ring, GraticuleRingMaterial};
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
    kavrayskiy_vii_graticule_materials: &mut ResMut<Assets<KavrayskiyViiGraticuleMaterial>>,
    position: Vec3,
    parallels: Vec<f32>,
    meridians: Vec<f32>,
    projection_fn: impl Fn(f32, f32) -> (f32, f32),
) {
    // Meridians
    {};

    // Parallels
    {};

    // Indicators
    {};

    // Notes
    {};
}
