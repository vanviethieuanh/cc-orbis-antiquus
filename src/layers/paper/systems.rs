use bevy::prelude::*;

use crate::constant::PAPER_Z_INDEX;

pub fn setup_paper_system(mut commands: Commands) {
    commands.spawn((
        Sprite::default(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, PAPER_Z_INDEX)),
    ));
}
