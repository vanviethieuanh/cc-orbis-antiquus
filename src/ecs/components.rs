use bevy::prelude::*;

#[derive(Component)]
pub struct LayerType {
    pub layer: MapLayer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapLayer {
    Paper,
    Map,
    Overlays,
    Outlines,
}

// Phase tracking markers
#[derive(Component)]
pub struct HasSolidColorBlock;

#[derive(Component)]
pub struct HasStyling;

#[derive(Component)]
pub struct HasFXShaders;

#[derive(Component)]
pub struct IsDynamic;
