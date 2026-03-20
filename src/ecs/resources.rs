use bevy::prelude::*;
use crate::cli::Cli;

#[derive(Resource)]
pub struct MapSettings {
    pub cli: Cli,
}
