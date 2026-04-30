use bevy::prelude::*;

// TODO: Remove this file
pub(crate) fn draw_ring(
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
