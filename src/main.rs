mod cli;
mod ecs;
mod layers;
mod palette;
mod render;
mod setup;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use bevy::window::WindowResolution;
use clap::Parser;
use cli::Cli;
use ecs::MapSettings;
use layers::map::setup_map_system;
use layers::outlines::setup_outlines_system;
use layers::overlays::setup_overlays_system;
use layers::paper::setup_paper_system;
use setup::setup_camera;

use crate::render::primitives::circle::CircleMaterial;

fn main() {
    let cli = Cli::parse();
    let (width, height) = cli.compute_window_size();

    let background_color = Color::srgb(230.0 / 255.0, 211.0 / 255.0, 169.0 / 255.0);

    App::new()
        .insert_resource(ClearColor(background_color))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(width, height),
                    title: "cc-orbis-antiquus".to_string(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CircleMaterial>::default(),
        ))
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
        .run();
}
