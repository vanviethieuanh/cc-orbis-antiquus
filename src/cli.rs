use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "cc-orbis-antiquus")]
#[command(about = "Render historical world maps", long_about = None)]
pub struct Cli {
    /// Input shapefile (.shp)
    #[arg(
        short,
        long,
        default_value = "data/raw/natural_earth/ne_50m_land/ne_50m_land.shp"
    )]
    pub input: String,

    /// Window width / render resolution
    #[arg(long)]
    pub width: Option<u32>,

    /// Window height / render resolution
    #[arg(long)]
    pub height: Option<u32>,

    /// Earth radius (controls map size)
    #[arg(long, default_value_t = 560.0)]
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

impl Cli {
    pub fn compute_window_size(&self) -> (u32, u32) {
        let base_size = ((self.radius + self.margin) * 2.0) as u32;
        let width = self.width.unwrap_or(base_size);
        let height = self.height.unwrap_or(base_size);
        (width, height)
    }
}
