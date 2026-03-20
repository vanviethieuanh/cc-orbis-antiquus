use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "map-art")]
#[command(about = "Render map projection to image", long_about = None)]
pub struct Cli {
    /// Input shapefile (.shp)
    #[arg(
        short,
        long,
        default_value = "data/raw/natural_earth/ne_50m_land/ne_50m_land.shp"
    )]
    pub input: String,

    /// Output image path
    #[arg(short, long, default_value = "out/world_map.png")]
    pub output: String,

    /// Earth radius (controls image size)
    #[arg(long, default_value_t = 1000.0)]
    pub radius: f32,

    /// Margin around the map
    #[arg(long, default_value_t = 120.0)]
    pub margin: f32,

    /// Perspective distance factor (D = d * R)
    #[arg(long, default_value_t = 5.62)]
    pub distance: f32,

    /// Only render northern hemisphere
    #[arg(long, default_value_t = true)]
    pub north_only: bool,
}
