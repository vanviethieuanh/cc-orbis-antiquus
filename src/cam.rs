use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
}
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
        Msaa::Off,
    ));
}

pub fn zoom_camera(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.zoom_speed;
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.zoom_range.start,
                camera_settings.zoom_range.end,
            );
        }
        _ => (),
    }
}

pub fn move_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<&mut Transform, With<Camera>>,
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

    let speed = 500.0; // tweak this
    transform.translation += direction * speed * time.delta_secs();
}
