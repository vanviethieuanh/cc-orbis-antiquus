use super::components::CircleGraticuleGrid;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

const CIRCLE_SEGMENTS: usize = 128;

pub fn setup_circle_graticule_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: &CircleGraticuleGrid,
    parallel_ratio_fn: impl Fn(f32) -> f32,
    z_index: f32,
) {
    // Draw meridians (straight lines from highest parallel to outer circle, or from center if no parallels)
    draw_meridians(
        &mut commands,
        &mut meshes,
        &mut materials,
        grid.radius,
        grid.meridians,
        grid.meridian_color,
        &grid.parallels,
        &parallel_ratio_fn,
        z_index,
    );

    // Draw parallels (concentric circles)
    draw_parallels(
        &mut commands,
        &mut meshes,
        &mut materials,
        &grid.parallels,
        grid.radius,
        grid.parallel_color,
        parallel_ratio_fn,
        z_index,
    );

    // Draw outer boundary circle
    draw_outer_circle(
        &mut commands,
        &mut meshes,
        &mut materials,
        grid.radius,
        grid.meridian_color,
        z_index,
    );
}

fn draw_outer_circle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    radius: f32,
    color: Color,
    z_index: f32,
) {
    let mut circle_positions = Vec::new();
    for i in 0..=CIRCLE_SEGMENTS {
        let angle = (i as f32 / CIRCLE_SEGMENTS as f32) * std::f32::consts::TAU;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        circle_positions.push(Vec3::new(x, y, 0.0));
    }

    if !circle_positions.is_empty() {
        let mut circle_mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        );
        circle_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, circle_positions);

        let mesh_handle = meshes.add(circle_mesh);
        let material_handle = materials.add(ColorMaterial::from_color(color));

        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::default().with_translation(Vec3::new(0.0, 0.0, z_index)),
        ));
    }
}

fn draw_parallels(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parallels: &[f32],
    outer_radius: f32,
    color: Color,
    parallel_ratio_fn: impl Fn(f32) -> f32,
    z_index: f32,
) {
    for &latitude in parallels {
        let parallel_radius = parallel_ratio_fn(latitude) * outer_radius;

        let mut parallel_positions = Vec::new();
        for j in 0..=CIRCLE_SEGMENTS {
            let angle = (j as f32 / CIRCLE_SEGMENTS as f32) * std::f32::consts::TAU;
            let x = parallel_radius * angle.cos();
            let y = parallel_radius * angle.sin();
            parallel_positions.push(Vec3::new(x, y, 0.0));
        }

        if !parallel_positions.is_empty() {
            let mut parallel_mesh = Mesh::new(
                PrimitiveTopology::LineStrip,
                RenderAssetUsages::RENDER_WORLD,
            );
            parallel_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, parallel_positions);

            let mesh_handle = meshes.add(parallel_mesh);
            let material_handle = materials.add(ColorMaterial::from_color(color));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::default().with_translation(Vec3::new(0.0, 0.0, z_index)),
            ));
        }
    }
}

fn draw_meridians(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    radius: f32,
    meridian_count: u32,
    color: Color,
    parallels: &[f32],
    parallel_ratio_fn: &dyn Fn(f32) -> f32,
    z_index: f32,
) {
    // Determine starting radius: if parallels exist, start from the highest latitude parallel; otherwise start from center
    let start_radius = if let Some(&highest_latitude) = parallels.last() {
        parallel_ratio_fn(highest_latitude) * radius
    } else {
        0.0
    };

    for i in 0..meridian_count {
        let angle = (i as f32 / meridian_count as f32) * std::f32::consts::TAU;
        let x_outer = radius * angle.sin();
        let y_outer = -radius * angle.cos();
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

        let mesh_handle = meshes.add(line_mesh);
        let material_handle = materials.add(ColorMaterial::from_color(color));

        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::default().with_translation(Vec3::new(0.0, 0.0, z_index)),
        ));
    }
}
