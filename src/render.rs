use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

pub fn draw_segment(
    img: &mut RgbaImage,
    prev: Option<(f32, f32, bool)>,
    curr: (f32, f32, bool),
    margin: f32,
) -> Option<(f32, f32, bool)> {
    if let Some((px, py, pvis)) = prev {
        let (cx, cy, cvis) = curr;

        if pvis || cvis {
            draw_line_segment_mut(
                img,
                (px + margin, py + margin),
                (cx + margin, cy + margin),
                Rgba([0, 0, 0, 255]),
            );
        }
    }

    Some(curr)
}
