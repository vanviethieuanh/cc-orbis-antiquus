use bevy::{ecs::resource::Resource, math::Vec2};

use crate::constants::{MAIN_NOTE, TITLE};

#[derive(Debug, Clone, Resource)]
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ZIndexConfig {
    pub paper: f32,
    pub map: f32,
    pub overlays: f32,
}

impl Default for ZIndexConfig {
    fn default() -> Self {
        Self {
            paper: 1.0,
            map: 2.0,
            overlays: 3.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PolarConfig {
    pub radius: f32,
    pub ring_thickness: f32,
    pub ring_spacing_factor: f32,
    pub stroke_thickness: f32,
    pub lim_lat: f32,
}

impl Default for PolarConfig {
    fn default() -> Self {
        Self {
            radius: 560.0,
            ring_thickness: 6.0,
            ring_spacing_factor: 3.0,
            stroke_thickness: 0.5,
            lim_lat: 30.0,
        }
    }
}

impl PolarConfig {
    pub fn offset(&self) -> f32 {
        self.ring_thickness * self.ring_spacing_factor
    }
}

#[derive(Debug, Clone)]
pub struct MainMapConfig {
    pub median_segments: usize,
    pub borders_spacings: Vec<f32>,
}

impl Default for MainMapConfig {
    fn default() -> Self {
        Self {
            median_segments: 64,
            borders_spacings: vec![12.0, 36.0, 60.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataConfig {
    pub shape_filepath: &'static str,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            shape_filepath: "data/raw/natural_earth/ne_110m_land/ne_110m_land.shp",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoteSpacingConfig {
    pub main: f32,
}

impl Default for NoteSpacingConfig {
    fn default() -> Self {
        Self { main: 10.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineThickness {
    Small,
    Medium,
    Large,
}

impl LineThickness {
    pub fn value(&self) -> f32 {
        match self {
            LineThickness::Small => 0.5,
            LineThickness::Medium => 1.0,
            LineThickness::Large => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineConfig {
    pub border: LineThickness,
    pub divider: LineThickness,
    pub accent: LineThickness,
}

impl Default for LineConfig {
    fn default() -> Self {
        Self {
            border: LineThickness::Medium,
            divider: LineThickness::Large,
            accent: LineThickness::Medium,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteConfig {
    pub font_size: f32,
    pub text: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct FontConfig {
    pub bold: &'static str,
    pub regular: &'static str,
    pub light: &'static str,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            regular: "fonts/LXGWWenKaiTC-Regular.ttf",
            bold: "fonts/LXGWWenKaiTC-Bold.ttf",
            light: "fonts/LXGWWenKaiTC-Light.ttf",
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotesConfig {
    pub font: FontConfig,
    pub spacing: NoteSpacingConfig,
    pub lines: LineConfig,
    pub title: NoteConfig,
    pub main_note: NoteConfig,
}

#[derive(Debug, Resource)]
pub struct MapConfig {
    pub canvas: CanvasConfig,
    pub z: ZIndexConfig,
    pub polar: PolarConfig,
    pub main_map: MainMapConfig,
    pub data: DataConfig,
    pub note: NotesConfig,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig {
                size: Vec2::new(11100., 5000.),
                border_thickness: 20.,
                margin: Edges {
                    top: 40.,
                    right: 40.,
                    bottom: 40.,
                    left: 40.,
                },
            },
            z: ZIndexConfig::default(),
            polar: PolarConfig::default(),
            main_map: MainMapConfig::default(),
            data: DataConfig::default(),
            note: NotesConfig {
                font: FontConfig::default(),
                spacing: NoteSpacingConfig::default(),
                lines: LineConfig::default(),
                title: NoteConfig {
                    font_size: 160.0,
                    text: TITLE,
                },
                main_note: NoteConfig {
                    font_size: 28.0,
                    text: MAIN_NOTE,
                },
            },
        }
    }
}
