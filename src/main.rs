mod cli;
mod overlay;
mod projections;
mod render;

use ab_glyph::{FontRef, PxScale};
use clap::Parser;
use cli::Cli;
use image::{Rgba, RgbaImage};
use shapefile::Reader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let r_earth = args.radius;
    let margin = args.margin;
    let d = args.distance;

    let font = FontRef::try_from_slice(include_bytes!("../data/fonts/LXGWWenKaiTC-Regular.ttf"))?;
    let font_scale = PxScale::from(20.0);

    let size = ((r_earth + margin) * 2.0) as u32;
    let mut img = RgbaImage::new(size, size);

    for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }

    let mut reader = Reader::from_path(&args.input)?;

    for result in reader.iter_shapes_and_records() {
        let (shape, _) = result?;

        if let shapefile::Shape::Polygon(poly) = shape {
            for ring in poly.rings() {
                let mut prev = None;

                for point in ring.points() {
                    let lon = point.x as f32;
                    let lat = point.y as f32;

                    if args.north_only && lat < 0.0 {
                        continue;
                    }

                    let p = projections::perspective_pole(r_earth, lon, lat, d);

                    prev = render::draw_segment(&mut img, prev, (p.x, p.y, p.visible), margin);
                }
            }
        }
    }

    let r_proj = projections::max_projected_radius(r_earth, d);
    overlay::draw_degree_ring(
        &mut img,
        r_proj,
        margin + (r_earth - r_proj),
        36,
        16.0,
        font_scale,
        &font,
        r_earth,
        d,
    );

    img.save(&args.output)?;
    println!("Saved {}", args.output);

    Ok(())
}
