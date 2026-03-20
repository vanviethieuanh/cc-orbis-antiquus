use bevy::prelude::*;

pub fn setup_outlines_system(mut commands: Commands) {
    // Phase 1: Stub - to be implemented with z-index 3 (top)
    // Will create outline frame, internal dividers, region markers
    commands.spawn((
        Sprite::default(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, 3.0)),
    ));
}
