mod projections;

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use proj::Proj;
use shapefile::Reader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Image canvas
    let r_earth = 1000f32;
    let margin = 120f32;
    let d = 5.62;

    let img_w = ((r_earth + margin) * 2f32) as u32;
    let img_h = ((r_earth + margin) * 2f32) as u32;
    let mut img = RgbImage::new(img_w, img_h);

    // Background color white
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Winkel Tripel projection (lon/lat → x/y)
    let proj = Proj::new_known_crs("EPSG:4326", "ESRI:54042", None)?; // ESRI:54042 = Winkel Tripel

    let mut reader = Reader::from_path("data/raw/natural_earth/ne_110m_land/ne_110m_land.shp")?;

    for result in reader.iter_shapes_and_records() {
        let (shape, _record) = result?;
        if let shapefile::Shape::Polygon(poly) = shape {
            for ring in poly.rings() {
                let mut prev: Option<(f32, f32)> = None;

                for point in ring.points() {
                    println!("Geo point: {}", point);

                    let long = point.x as f32;
                    let lat = point.y as f32;

                    if lat < 0.0 {
                        continue;
                    }

                    let (px, py) = projections::draw_pole(r_earth, long, lat, d);

                    println!("Projected: {}, {}", px, py);

                    if let Some(prev_pt) = prev {
                        let (prev_x, prev_y) = prev_pt;

                        draw_line_segment_mut(
                            &mut img,
                            (prev_x + margin, prev_y + margin),
                            (px + margin, py + margin),
                            Rgb([0, 0, 0]),
                        );
                    }

                    prev = Some((px, py));
                }

                if let (Some(first), Some(last)) = (ring.points().first(), prev) {
                    let first_projected =
                        projections::draw_pole(r_earth, first.x as f32, first.y as f32, d);

                    draw_line_segment_mut(
                        &mut img,
                        (last.0 + margin, last.1 + margin),
                        (first_projected.0 + margin, first_projected.1 + margin),
                        Rgb([0, 0, 0]),
                    );
                }
            }
        }
    }

    img.save("world_map.jpg")?;
    println!("Saved world_map.jpg");

    Ok(())
}
