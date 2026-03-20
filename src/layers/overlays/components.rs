use bevy::prelude::*;

use crate::palette::PARCHMENT_INK;

#[derive(Component)]
pub struct CircleGraticuleGrid {
    pub radius: f32,
    pub meridians: u32,
    pub parallels: Vec<f32>,
    pub boundary_color: Color,
    pub meridian_color: Color,
    pub parallel_color: Color,
    pub boundary_thickness: f32,
    pub graticule_ring_thickness: f32,
}

impl CircleGraticuleGrid {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            meridians: 36,
            parallels: vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0],
            meridian_color: PARCHMENT_INK,
            parallel_color: PARCHMENT_INK,
            boundary_color: PARCHMENT_INK,
            boundary_thickness: 1.0,
            graticule_ring_thickness: 6.0,
        }
    }

    pub fn with_meridians(mut self, count: u32) -> Self {
        self.meridians = count;
        self
    }

    pub fn with_parallels(mut self, latitudes: Vec<f32>) -> Self {
        self.parallels = latitudes;
        self
    }

    pub fn with_colors(mut self, meridian_color: Color, parallel_color: Color) -> Self {
        self.meridian_color = meridian_color;
        self.parallel_color = parallel_color;
        self
    }

    pub fn with_line_thickness(mut self, thickness: f32) -> Self {
        self.boundary_thickness = thickness;
        self
    }
}

#[derive(Component)]
pub struct TextContent {
    pub text: String,
    pub language: TextLanguage,
}

#[derive(Clone, Copy, Debug)]
pub enum TextLanguage {
    Chinese,
    English,
}

#[derive(Component)]
pub struct TextStyle {
    pub font_size: f32,
    pub orientation: TextOrientation,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub enum TextOrientation {
    Horizontal,
    Vertical,
}

#[derive(Component)]
pub struct Diagram {
    pub diagram_type: DiagramType,
}

#[derive(Clone, Copy, Debug)]
pub enum DiagramType {
    OrbitExplanation,
    CelestialSphere,
}

#[derive(Component)]
pub struct DecorativeElement {
    pub element_type: DecorativeElementType,
}

#[derive(Clone, Copy, Debug)]
pub enum DecorativeElementType {
    CompassRose,
    CelestialDiagram,
}
