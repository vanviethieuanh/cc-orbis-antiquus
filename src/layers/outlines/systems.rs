use bevy::prelude::*;

use crate::{
    constant::{CANVAS_BORDER_THICKNESS, CANVAS_SIZE, OUTLINES_Z_INDEX},
    palette::PARCHMENT_INK,
};

pub fn setup_outlines_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let border =
        meshes.add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y).to_ring(CANVAS_BORDER_THICKNESS));
    commands.spawn((
        Mesh2d(border),
        MeshMaterial2d(color_materials.add(PARCHMENT_INK)),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, OUTLINES_Z_INDEX)),
    ));
}
