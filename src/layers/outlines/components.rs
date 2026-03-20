use bevy::prelude::*;

#[derive(Component)]
pub struct OutlineFrame {
    pub shape: FrameShape,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub enum FrameShape {
    Oval,
    Circle,
}

#[derive(Component)]
pub struct RegionMarker {
    pub region_type: RegionType,
}

#[derive(Clone, Copy, Debug)]
pub enum RegionType {
    TitleBlock,
    NoteArea,
    DiagramArea,
    MapArea,
}

#[derive(Component)]
pub struct InternalDivider;
