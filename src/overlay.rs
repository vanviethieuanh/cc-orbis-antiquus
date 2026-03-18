use ab_glyph::{Font, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_hollow_circle_mut, draw_line_segment_mut, draw_polygon_mut, draw_text_mut},
    geometric_transformations::{Interpolation, rotate_about_center},
    point::Point,
};
use std::f32::consts::PI;

use crate::projections::parallel_ratio;

pub fn draw_degree_ring(
    img: &mut RgbaImage,
    r: f32,
    margin: f32,
    sections: usize,
    thickness: f32,
    scale: impl Into<PxScale> + Copy,
    font: &impl Font,

    r_earth: f32,
    d: f32,
) {
    let cx = r + margin;
    let cy = r + margin;
    let center = (cx as i32, cy as i32);
    let full = 2.0 * PI;

    // Draw alternating black/white arc segments
    for i in 0..360 {
        let theta0 = i as f32 / 360.0 * full;
        let theta1 = (i + 1) as f32 / 360.0 * full;

        let color = if i % 2 == 0 {
            Rgba([0, 0, 0, 255])
        } else {
            Rgba([255, 255, 255, 255])
        };

        fill_arc_segment(img, cx, cy, r, r + thickness, theta0, theta1, color);
    }

    let parallel_80th_r = parallel_ratio(80.0, r, d) * r_earth;
    for i in 5..9 {
        let parallel_r = parallel_ratio(i as f32 * 10.0, r, d) * r_earth;
        draw_hollow_circle_mut(img, center, parallel_r as i32, Rgba([120, 120, 120, 255]));
    }

    // Draw section lines and notes
    for i in 0..sections {
        let theta = i as f32 / sections as f32 * full;

        let (x0, y0) = polar(cx, cy, parallel_80th_r, theta);
        let (x1, y1) = polar(cx, cy, r + 4.0 * thickness, theta);

        draw_line_segment_mut(img, (x0, y0), (x1, y1), Rgba([120, 120, 120, 255]));

        // Note text
        let mid_r = r + 2.0 * thickness;
        let (tx, ty) = polar(cx, cy, mid_r, full - theta - PI / 18.0 + PI / 144.0);

        let note = match i {
            0 => "一十",
            1 => "二十",
            2 => "三十",
            3 => "四十",
            4 => "五十",
            5 => "六十",
            6 => "七十",
            7 => "八十",
            8 => "九十",
            9 => "一百",
            10 => "一百一",
            11 => "一百卄",
            12 => "一百卅",
            13 => "一百卌",
            14 => "一百五",
            15 => "一百六",
            16 => "一百七",
            17 => "一百八",
            18 => "一百九",
            19 => "二百",
            20 => "二百一",
            21 => "二百卄",
            22 => "二百卅",
            23 => "二百卌",
            24 => "二百五",
            25 => "二百六",
            26 => "二百七",
            27 => "二百八",
            28 => "二百九",
            29 => "三百",
            30 => "三百一",
            31 => "三百卄",
            32 => "三百卅",
            33 => "三百卌",
            34 => "三百五",
            35 => "三百六",
            _ => "",
        };

        // Prepare a transparent image slightly taller for vertical text
        let char_height = scale.into().y as u32;
        let char_width = scale.into().x as u32;
        let text_width = char_width + 50;
        let text_height = char_height * note.chars().count() as u32;
        let mut note_img = RgbaImage::from_pixel(text_width, text_height, Rgba([0, 0, 0, 0]));

        let spacing_factor = 0.6; // 0.8 = 80% of the font height
        let step = (char_height as f32 * spacing_factor) as i32;

        // Draw each character vertically
        for (idx, c) in note.chars().enumerate() {
            draw_text_mut(
                &mut note_img,
                Rgba([0, 0, 0, 255]),
                25,
                idx as i32 * step,
                scale,
                font,
                &c.to_string(),
            );
        }

        let angle = full - theta - PI / 18.0 + PI / 144.0;
        let rotated = rotate_about_center(
            &note_img,
            angle,
            Interpolation::Bilinear,
            Rgba([0, 0, 0, 0]),
        );

        let px = tx - rotated.width() as f32 / 2.0;
        let py = ty - rotated.height() as f32 / 2.0;

        image::imageops::overlay(img, &rotated, px as i64, py as i64);
    }

    // Main title
    draw_text_mut(
        img,
        Rgba([0, 0, 0, 255]),
        (cx - 3.5 * 80.0) as i32,
        (cy - r - 3.0 * thickness - margin / 2.0) as i32,
        PxScale::from(80.0),
        font,
        "圖 之 球 地 北 道 赤",
    );

    // Radial borders
    draw_hollow_circle_mut(
        img,
        center,
        (r + 4.0 * thickness) as i32,
        Rgba([0, 0, 0, 255]),
    );
    draw_hollow_circle_mut(img, center, (r + thickness) as i32, Rgba([0, 0, 0, 255]));
    draw_hollow_circle_mut(img, center, r as i32, Rgba([0, 0, 0, 255]));
}

fn fill_arc_segment(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    r_inner: f32,
    r_outer: f32,
    theta0: f32,
    theta1: f32,
    color: Rgba<u8>,
) {
    let steps = 64.max(((theta1 - theta0).abs() * 180.0 / PI) as usize);

    let mut points: Vec<Point<i32>> = Vec::with_capacity(steps * 2);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let theta = theta0 + t * (theta1 - theta0);
        let (x, y) = polar(cx, cy, r_outer, theta);
        points.push(Point::new(x as i32, y as i32));
    }
    for i in (0..=steps).rev() {
        let t = i as f32 / steps as f32;
        let theta = theta0 + t * (theta1 - theta0);
        let (x, y) = polar(cx, cy, r_inner, theta);
        points.push(Point::new(x as i32, y as i32));
    }

    draw_polygon_mut(img, &points, color);
}

fn polar(cx: f32, cy: f32, r: f32, theta: f32) -> (f32, f32) {
    let x = cx + r * theta.sin();
    let y = cy - r * theta.cos();
    (x, y)
}
