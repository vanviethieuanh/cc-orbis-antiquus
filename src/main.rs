mod cam;
mod cli;
mod constant;
mod ecs;
mod layers;
mod palette;
mod render;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use clap::Parser;
use cli::Cli;
use ecs::MapSettings;
use layers::map::setup_map_system;
use layers::outlines::setup_outlines_system;
use layers::overlays::setup_overlays_system;
use layers::paper::setup_paper_system;

use crate::cam::{move_camera, setup_camera, zoom_camera, CameraSettings};
use crate::palette::PARCHMENT_BG;
use crate::render::indicator::GraticuleRingMaterial;
use crate::render::primitives::circle::CircleMaterial;

fn main() {
    let cli = Cli::parse();

    App::new()
        .insert_resource(ClearColor(PARCHMENT_BG))
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
        ))
        .insert_resource(CameraSettings {
            zoom_range: 0.8..10.,
            zoom_speed: 0.2,
            move_speed: 1000.,
        })
        .insert_resource(MapSettings { cli })
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_outlines_system,
                setup_map_system,
                setup_overlays_system,
                setup_paper_system,
            ),
        )
        .add_systems(Update, (zoom_camera, move_camera))
        .run();
}
