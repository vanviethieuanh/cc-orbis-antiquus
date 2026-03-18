use shapefile::Reader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = "data/raw/natural-earth/ne_110m_land/ne_110m_land";
    let shp = format!("{}.shp", base);
    let shx = format!("{}.shx", base);
    let dbf = format!("{}.dbf", base);

    for file in [&shp, &shx, &dbf] {
        if !Path::new(file).exists() {
            panic!("Required shapefile component missing: {}", file);
        }
    }

    let mut reader = Reader::from_path(&shp)?;

    for (i, result) in reader.iter_shapes_and_records().enumerate() {
        let (shape, record) = result.unwrap();
        println!("Shape #{}: {:?}, records:", i, shape.to_string());
        match shape {
            shapefile::Shape::Polygon(poly) => {
                println!(
                    "Shape #{}: Polygon with {} points",
                    i,
                    poly.total_point_count()
                );
            }
            _ => {
                println!("Shape #{}: Other shape: {:?}", i, shape.shapetype());
            }
        }

        for (name, value) in record {
            println!("\t{}: {:?}", name, value);
        }
        println!();
    }

    Ok(())
}
