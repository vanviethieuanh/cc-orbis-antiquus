use bevy::math::Vec2;

pub static CANVAS_SIZE: Vec2 = Vec2::new(10000., 5000.);
pub static CANVAS_BORDER_THICKNESS: f32 = 20.;
pub static CANVAS_MARGIN: (f32, f32, f32, f32) = (20., 40., 20., 40.);

pub static CANVAS_LEFT: f32 = -(CANVAS_SIZE.x) / 2. + CANVAS_BORDER_THICKNESS + CANVAS_MARGIN.3;
pub static CANVAS_TOP: f32 = (CANVAS_SIZE.y) / 2. - CANVAS_BORDER_THICKNESS - CANVAS_MARGIN.0;
pub static CANVAS_BOT: f32 = -(CANVAS_SIZE.y) / 2. + CANVAS_BORDER_THICKNESS + CANVAS_MARGIN.2;

pub static PAPER_Z_INDEX: f32 = 1.0;
pub static MAP_Z_INDEX: f32 = 2.0;
pub static OVERLAYS_Z_INDEX: f32 = 3.0;

pub static POLARS_RADIUS: f32 = 560.0;
// Actual Distance = POLARS_RADIUS * this value;
pub static POLE_VIEWPOINT_DISTANCE: f32 = 5.62;

pub static MAIN_MAP_MEDIAN_SEGMENTS: usize = 128;

pub static MAP_SHAPE_FILEPATH: &str = "data/raw/natural_earth/ne_110m_land/ne_110m_land.shp";
