use bevy::prelude::*;

#[derive(Component)]
pub struct GraticuleGrid {
    pub spacing: f32,
    pub color: Color,
    pub density: i32,
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
