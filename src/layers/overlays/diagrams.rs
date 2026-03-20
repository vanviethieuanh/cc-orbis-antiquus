pub struct OverlayConfig {
    pub title: String,
    pub section_labels: Vec<String>,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            title: "圖 之 球 地 北 道 赤".to_string(),
            section_labels: vec![
                "一十",
                "二十",
                "三十",
                "四十",
                "五十",
                "六十",
                "七十",
                "八十",
                "九十",
                "一百",
                "一百一",
                "一百卄",
                "一百卅",
                "一百卌",
                "一百五",
                "一百六",
                "一百七",
                "一百八",
                "一百九",
                "二百",
                "二百一",
                "二百卄",
                "二百卅",
                "二百卌",
                "二百五",
                "二百六",
                "二百七",
                "二百八",
                "二百九",
                "三百",
                "三百一",
                "三百卄",
                "三百卅",
                "三百卌",
                "三百五",
                "三百六",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }
}
