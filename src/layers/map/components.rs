use bevy::prelude::*;

#[derive(Component)]
pub struct MapRegion {
    pub projection_type: ProjectionType,
    pub bounds: Rect,
}

#[derive(Clone, Copy, Debug)]
pub enum ProjectionType {
    WinkelTripel,
    Azimuthal,
}

#[derive(Component)]
pub struct ProjectionData {
    pub projection_type: ProjectionType,
    pub cached_coords: Vec<Vec2>,
}

#[derive(Component)]
pub struct WaterGeometry;

#[derive(Component)]
pub struct LandGeometry;

#[derive(Component)]
pub struct LakeGeometry;

#[derive(Component)]
pub struct RiverGeometry;

#[derive(Component)]
pub struct MountainGeometry;
