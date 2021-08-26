pub struct MapElementUpdate {
    pub background: String,
    pub foreground: String,
    pub symbol: char,
}

pub struct ScreenUpdate {
    // A map showing the area around the hero.
    pub map: Vec<Vec<MapElementUpdate>>,
}
