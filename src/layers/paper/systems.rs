use bevy::prelude::*;

use crate::config::MapConfig;
use crate::palette::ColorTheme;
use crate::render::paper::{ParchmentBgMaterial, ParchmentTextureParams};

pub fn setup_paper_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParchmentBgMaterial>>,
    map_config: Res<MapConfig>,
    color_theme: Res<ColorTheme>,
) {
    let size = map_config.canvas.size;

    let mesh_ratio = Vec2::new(size.x / size.y, 1.0);

    let params = ParchmentTextureParams {
        parchment_bg: LinearRgba::from(color_theme.parchment.bg).to_vec4(),
        parchment_light: LinearRgba::from(color_theme.parchment.light).to_vec4(),
        parchment_dark: LinearRgba::from(color_theme.parchment.dark).to_vec4(),
        mesh_ratio,
        ..Default::default()
    };

    let material = materials.add(ParchmentBgMaterial { params });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(material),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, map_config.z.paper)),
    ));
}
