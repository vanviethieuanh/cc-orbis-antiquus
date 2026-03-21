use bevy::math::{Vec2, Vec4};

pub static CANVAS_SIZE: Vec2 = Vec2::new(12000., 5000.);
pub static CANVAS_BORDER_THICKNESS: f32 = 20.;
pub static CANVAS_MARGIN: Vec4 = Vec4::new(20., 40., 20., 40.);

pub static PAPER_Z_INDEX: f32 = 1.0;
pub static OUTLINES_Z_INDEX: f32 = 2.0;
pub static MAP_Z_INDEX: f32 = 3.0;
pub static OVERLAYS_Z_INDEX: f32 = 4.0;
