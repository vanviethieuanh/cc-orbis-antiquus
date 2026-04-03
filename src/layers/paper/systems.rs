use bevy::prelude::*;

use crate::config::MapConfig;

pub fn setup_paper_system(mut commands: Commands, map_config: Res<MapConfig>) {
    commands.spawn((
        Sprite::default(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, map_config.z.paper)),
    ));
}
