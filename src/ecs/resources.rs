use std::error::Error;

use crate::{cli::Cli, config::MapConfig};
use bevy::prelude::*;
use shapefile::{Reader, Shape};

#[derive(Resource)]
pub struct MapData {
    pub polylines: Vec<Vec<Vec2>>,
}

pub fn load_map(map_config: &MapConfig) -> MapData {
    let mut reader =
        Reader::from_path(map_config.data.shape_filepath).expect("failed to open shapefile");

    let mut polylines = Vec::new();

    for result in reader.iter_shapes_and_records() {
        match result {
            Ok((shape, _record)) => {
                if let shapefile::Shape::Polygon(poly) = shape {
                    for ring in poly.rings() {
                        polylines.push(
                            ring.points()
                                .iter()
                                .map(|p| Vec2::new(p.x as f32, p.y as f32))
                                .collect(),
                        );
                    }
                }
            }
            Err(err) => error!("{}", err),
        };
    }

    MapData { polylines }
}

#[derive(Resource)]
pub struct FontAssets {
    pub bold: Handle<Font>,
    pub regular: Handle<Font>,
    pub light: Handle<Font>,
}

pub fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_config: Res<MapConfig>,
) {
    commands.insert_resource(FontAssets {
        bold: asset_server.load(map_config.note.font.bold),
        regular: asset_server.load(map_config.note.font.regular),
        light: asset_server.load(map_config.note.font.light),
    });
}
