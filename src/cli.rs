use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "cc-orbis-antiquus")]
#[command(about = "Render historical world maps", long_about = None)]
pub struct Cli {
    /// Input shapefile (.shp)
    #[arg(
        short,
        long,
        default_value = "data/raw/natural_earth/ne_110m_land/ne_110m_land.shp"
    )]
    pub input: String,

    /// Window width / render resolution
    #[arg(long)]
    pub width: Option<u32>,

    /// Window height / render resolution
    #[arg(long)]
    pub height: Option<u32>,

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
