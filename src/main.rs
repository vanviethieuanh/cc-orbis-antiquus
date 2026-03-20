mod cli;
mod overlay;
mod projections;
mod render_bevy;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use clap::Parser;
use cli::Cli;
use render_bevy::{setup_camera, MapSettings};

fn main() {
    let cli = Cli::parse();
    let (width, height) = cli.compute_window_size();

    let background_color = Color::srgb(230.0 / 255.0, 211.0 / 255.0, 169.0 / 255.0);

    App::new()
        .insert_resource(ClearColor(background_color))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(width, height),
                title: "cc-orbis-antiquus".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(MapSettings { cli })
        .add_systems(Startup, (setup_camera, setup_map_wrapper))
        .run();
}

fn setup_map_wrapper(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<MapSettings>,
) {
    if let Err(e) = render_bevy::setup_map(commands, meshes, materials, settings) {
        eprintln!("Error loading map: {}", e);
    }
}
