use bevy::math::Vec2;

pub static CANVAS_SIZE: Vec2 = Vec2::new(12000., 5000.);
pub static CANVAS_BORDER_THICKNESS: f32 = 20.;
pub static CANVAS_MARGIN: (f32, f32, f32, f32) = (20., 40., 20., 40.);

pub static CANVAS_LEFT: f32 = -(CANVAS_SIZE.x) / 2. + CANVAS_BORDER_THICKNESS + CANVAS_MARGIN.3;
pub static CANVAS_TOP: f32 = (CANVAS_SIZE.y) / 2. - CANVAS_BORDER_THICKNESS - CANVAS_MARGIN.0;

pub static PAPER_Z_INDEX: f32 = 1.0;
pub static OUTLINES_Z_INDEX: f32 = 2.0;
pub static MAP_Z_INDEX: f32 = 3.0;
pub static OVERLAYS_Z_INDEX: f32 = 4.0;

pub static POLARS_RADIUS: f32 = 560.0;
