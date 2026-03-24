use std::error::Error;

use crate::cli::Cli;
use crate::constant::MAP_SHAPE_FILEPATH;
use bevy::prelude::*;
use shapefile::{Reader, Shape};

#[derive(Resource)]
pub struct MapData {
    pub polylines: Vec<Vec<Vec2>>,
}

pub fn load_map(mut commands: Commands) {
    let mut reader = Reader::from_path(MAP_SHAPE_FILEPATH).expect("failed to open shapefile");

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

    commands.insert_resource(MapData { polylines });
}
