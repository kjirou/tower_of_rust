//! A Data Transfer Object that aggregates the information required to update the screen.

use crate::enums::{ColorKind};

pub struct MapElementUpdate {
    pub background: ColorKind,
    pub foreground: ColorKind,
    pub symbol: char,
}

pub struct ScreenUpdate {
    pub fps: f64,
    pub last_key_input: String,
    pub number_of_frames: u64,
    /// A map showing the area around the hero.
    pub map: Vec<Vec<MapElementUpdate>>,
}
