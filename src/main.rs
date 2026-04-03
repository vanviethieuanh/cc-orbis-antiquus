mod cam;
mod cli;
mod config;
mod ecs;
mod layers;
mod palette;
mod projection;
mod render;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use layers::map::setup_map_system;
use layers::overlays::setup_overlays_system;
use layers::paper::setup_paper_system;

use crate::cam::{move_camera, setup_camera, zoom_camera, CameraSettings};
use crate::config::MapConfig;
use crate::ecs::load_map;
use crate::palette::ColorTheme;
use crate::render::graticule::indicator::GraticuleRingMaterial;
use crate::render::graticule::KavrayskiyViiGraticuleMaterial;
use crate::render::primitives::circle::CircleMaterial;

fn main() {
    let color_theme = ColorTheme::default();

    App::new()
        .insert_resource(ClearColor(color_theme.parchment.bg))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "cc-orbis-antiquus".to_string(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CircleMaterial>::default(),
            Material2dPlugin::<GraticuleRingMaterial>::default(),
            Material2dPlugin::<KavrayskiyViiGraticuleMaterial>::default(),
        ))
        .insert_resource(CameraSettings {
            zoom_range: 0.8..10.,
            zoom_speed: 0.2,
            move_speed: 1000.,
        })
        .insert_resource(MapConfig::default())
        .insert_resource(color_theme)
        .add_systems(
            Startup,
            (
                load_map,
                (
                    setup_camera,
                    setup_map_system,
                    setup_overlays_system,
                    setup_paper_system,
                )
                    .after(load_map),
            ),
        )
        .add_systems(Update, (zoom_camera, move_camera))
        .run();
}
