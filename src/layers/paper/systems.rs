use bevy::prelude::*;

pub fn setup_paper_system(mut commands: Commands) {
    // Phase 1: Placeholder background with z-index 0 (bottom)
    commands.spawn((
        Sprite::default(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
