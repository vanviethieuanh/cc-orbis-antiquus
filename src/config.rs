use bevy::{ecs::resource::Resource, math::Vec2};

#[derive(Debug, Resource)]
pub struct CanvasConfig {
    pub size: Vec2,
    pub border_thickness: f32,
    pub margin: Edges,
}
impl CanvasConfig {
    pub fn left(&self) -> f32 {
        -self.size.x / 2.0 + self.border_thickness
    }

    pub fn right(&self) -> f32 {
        self.size.x / 2.0 - self.border_thickness
    }

    pub fn top(&self) -> f32 {
        self.size.y / 2.0 - self.border_thickness
    }

    pub fn bottom(&self) -> f32 {
        -self.size.y / 2.0 + self.border_thickness
    }
}

#[derive(Debug, Resource)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Resource)]
pub struct ZIndexConfig {
    pub paper: f32,
    pub map: f32,
    pub overlays: f32,
}

#[derive(Debug, Resource)]
pub struct PolarConfig {
    pub radius: f32,

    pub ring_thickness: f32,
    pub ring_spacing_factor: f32,

    pub stroke_thickness: f32,
    pub lim_lat: f32,
}

impl PolarConfig {
    pub fn offset(&self) -> f32 {
        self.ring_thickness * self.ring_spacing_factor
    }
}

#[derive(Debug, Resource)]
pub struct MainMapConfig {
    pub median_segments: usize,
}

#[derive(Debug, Resource)]
pub struct DataConfig {
    pub shape_filepath: &'static str,
}

#[derive(Debug, Resource)]
pub struct MapConfig {
    pub canvas: CanvasConfig,
    pub z: ZIndexConfig,
    pub polar: PolarConfig,
    pub main_map: MainMapConfig,
    pub data: DataConfig,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig {
                size: Vec2::new(10000., 5000.),
                border_thickness: 20.,
                margin: Edges {
                    top: 40.,
                    right: 40.,
                    bottom: 40.,
                    left: 40.,
                },
            },
            z: ZIndexConfig {
                paper: 1.0,
                map: 2.0,
                overlays: 3.0,
            },
            polar: PolarConfig {
                radius: 560.0,

                ring_thickness: 6.0,
                ring_spacing_factor: 3.0,
                stroke_thickness: 0.5,

                lim_lat: 30.0,
            },
            main_map: MainMapConfig {
                median_segments: 128,
            },
            data: DataConfig {
                shape_filepath: "data/raw/natural_earth/ne_110m_land/ne_110m_land.shp",
            },
        }
    }
}
