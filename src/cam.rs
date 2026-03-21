use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,

    pub move_speed: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 10.,
            ..OrthographicProjection::default_2d()
        }),
        Msaa::Off,
    ));
}

pub fn zoom_camera(
    camera: Single<(&mut Projection, &mut Transform, &Camera, &GlobalTransform)>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    window: Single<&Window>,
) {
    let (mut projection, mut transform, camera_comp, camera_transform) = camera.into_inner();

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };

    let cursor_world = camera_comp
        .viewport_to_world_2d(camera_transform, cursor_pos)
        .unwrap();

    let old_scale = ortho.scale;

    let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.zoom_speed;
    let new_scale = (old_scale * (1. + delta_zoom)).clamp(
        camera_settings.zoom_range.start,
        camera_settings.zoom_range.end,
    );

    if new_scale == old_scale {
        return;
    }

    ortho.scale = new_scale;

    let cam_pos = transform.translation.truncate();
    let ratio = new_scale / old_scale;

    let new_cam_pos = cursor_world + (cam_pos - cursor_world) * ratio;

    transform.translation.x = new_cam_pos.x;
    transform.translation.y = new_cam_pos.y;
}

pub fn move_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
) {
    let mut transform = camera.into_inner();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * camera_settings.move_speed * time.delta_secs();
}
