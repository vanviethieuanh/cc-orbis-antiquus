use crate::config::MapConfig;
use crate::render::graticule::indicator::{spawn_graticule_ring, GraticuleRingMaterial};

use bevy::prelude::*;

pub fn setup_circle_graticule(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    graticule_ring_materials: &mut ResMut<Assets<GraticuleRingMaterial>>,
    map_config: &Res<MapConfig>,
    position: Vec3,
    parallel_ratio_fn: impl Fn(f32) -> f32,
    parallels: &Vec<f32>,
    meridians: usize,
    ink_color: Color,
) {
    // Meridians
    {
        let start_radius = if let Some(&highest_latitude) = parallels.last() {
            (&parallel_ratio_fn)(highest_latitude) * map_config.polar.radius
        } else {
            0.0
        };

        let end_radius = map_config.polar.radius + 3. * map_config.polar.ring_thickness;

        for i in 0..meridians {
            let angle = (i as f32 / meridians as f32) * std::f32::consts::TAU;

            let dir = Vec2::new(angle.sin(), -angle.cos());

            let start = dir * start_radius;
            let end = dir * end_radius;

            let length = (end - start).length();
            let mid = (start + end) * 0.5;

            let mesh_handle = meshes.add(Rectangle::new(1.0, 1.0));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(color_materials.add(ink_color)),
                Transform {
                    translation: (mid.extend(0.0) + position),
                    rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::new(map_config.polar.stroke_thickness, length, 1.0),
                },
            ));
        }
    };

    // Parallels
    {
        let outer_radius: f32 = map_config.polar.radius;
        for &latitude in parallels {
            draw_ring(
                commands,
                meshes,
                color_materials,
                parallel_ratio_fn(latitude) * outer_radius,
                map_config.polar.stroke_thickness,
                position,
                ink_color,
            );
        }
    };

    // First boundary circle
    draw_ring(
        commands,
        meshes,
        color_materials,
        map_config.polar.radius,
        map_config.polar.stroke_thickness,
        position,
        ink_color,
    );

    // spawn_graticule_ring
    spawn_graticule_ring(
        commands,
        meshes,
        graticule_ring_materials,
        position,
        // 0.9 make the right slightly tigher to inner border, avoid render leak create a
        //   slight white ring.
        2.0 * (map_config.polar.radius + map_config.polar.ring_thickness * 0.9),
        360.,
        map_config.polar.ring_thickness,
        0.5,
        0.025,
        ink_color.into(),
    );

    // Second boundary circle
    draw_ring(
        commands,
        meshes,
        color_materials,
        map_config.polar.radius + map_config.polar.ring_thickness,
        map_config.polar.stroke_thickness,
        position,
        ink_color,
    );

    // Third boundary circle
    draw_ring(
        commands,
        meshes,
        color_materials,
        map_config.polar.radius + 3.0 * map_config.polar.ring_thickness,
        map_config.polar.stroke_thickness,
        position,
        ink_color,
    );
}

pub fn setup_pseudocylindrical_graticule(
    commands: &mut Commands,
    map_config: &Res<MapConfig>,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    ratio: f32,
    meridians: Vec<f32>,
    parallels: Vec<f32>,
    color: Color,
    projection_fn: impl Fn(Vec2) -> Vec2,
) {
    {
        for border_spacing in &map_config.main_map.borders_spacings {
            // LEFT seam (-180)
            build_border_polyline(
                meshes,
                commands,
                color_materials,
                &projection_fn,
                &map_config,
                ratio,
                *border_spacing,
                -180.0,
                position,
                color,
            );

            // RIGHT seam (+180)
            build_border_polyline(
                meshes,
                commands,
                color_materials,
                &projection_fn,
                &map_config,
                ratio,
                *border_spacing,
                180.0,
                position,
                color,
            );
        }
    }

    // Meridians
    {
        for long in meridians {
            let med = (meshes).add(Polyline2d::new(
                (0..(map_config.main_map.median_segments + 1))
                    .map(|i| -90. + i as f32 * (180.0 / map_config.main_map.median_segments as f32))
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
}

fn draw_ring(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    color_materials: &mut ResMut<'_, Assets<ColorMaterial>>,
    radius: f32,
    stroke_thickness: f32,
    position: Vec3,
    ink_color: Color,
) {
    let mesh_handle = (meshes).add(
        Annulus::new(radius, radius + stroke_thickness)
            .mesh()
            .resolution(64)
            .build(),
    );
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(color_materials.add(ink_color)),
        Transform::default().with_translation(position),
    ));
}

fn liang_barsky_clip(p0: Vec2, p1: Vec2, min: Vec2, max: Vec2) -> Option<(Vec2, Vec2)> {
    let d = p1 - p0;

    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;

    let checks = [
        (-d.x, p0.x - min.x),
        (d.x, max.x - p0.x),
        (-d.y, p0.y - min.y),
        (d.y, max.y - p0.y),
    ];

    for (p, q) in checks {
        if p.abs() < 1e-6 {
            if q < 0.0 {
                return None; // parallel and outside
            }
        } else {
            let t = q / p;

            if p < 0.0 {
                // entering
                t0 = t0.max(t);
            } else {
                // exiting
                t1 = t1.min(t);
            }

            if t0 > t1 {
                return None;
            }
        }
    }

    Some((p0 + d * t0, p0 + d * t1))
}

fn build_border_polyline(
    meshes: &mut ResMut<Assets<Mesh>>,
    commands: &mut Commands,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
    projection_fn: impl Fn(Vec2) -> Vec2,
    map_config: &MapConfig,
    ratio: f32,
    border_spacing: f32,
    lon: f32,
    position: Vec3,
    color: Color,
) {
    let min = Vec2::new(map_config.canvas.left(), map_config.canvas.bottom());
    let max = Vec2::new(map_config.canvas.right(), map_config.canvas.top());

    let mut points = Vec::new();
    let mut prev: Option<Vec2> = None;

    for lat in (0..=map_config.main_map.median_segments)
        .map(|i| -90. + i as f32 * (180.0 / map_config.main_map.median_segments as f32))
    {
        let curr = projection_fn(Vec2::new(lon, lat)) * (ratio + border_spacing);

        if let Some(p0) = prev {
            let p1 = curr;

            if let Some((c0, c1)) = liang_barsky_clip(p0, p1, min, max) {
                if points
                    .last()
                    .map_or(true, |last: &Vec2| last.distance(c0) > 1e-4)
                {
                    points.push(c0);
                }
                points.push(c1);
            }
        }

        prev = Some(curr);
    }

    if points.len() < 2 {
        return;
    }

    let mesh = meshes.add(Polyline2d::new(points));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(color_materials.add(color)),
        Transform::default().with_translation(position),
    ));
}
