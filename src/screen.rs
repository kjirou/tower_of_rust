use std::collections::HashMap;
use termion::color;

use crate::enums::{ColorKind};
use crate::screen_update::ScreenUpdate;

#[derive(Clone)]
struct Cell {
    background: ColorKind,
    foreground: ColorKind,
    /// (x, y)
    position: (u8, u8),
    symbol: char,
}

pub struct Screen {
    background_map: HashMap<ColorKind, String>,
    foreground_map: HashMap<ColorKind, String>,
    matrix: [[Cell; 80]; 24],
}
impl Screen {
    pub fn new() -> Self {
        let default_cell = Cell {
            position: (0, 0),
            symbol: ' ',
            background: ColorKind::Black,
            foreground: ColorKind::White,
        };
        // TODO: 短くする。おそらくマクロでどうにかする気はする。または ndarray などのライブラリで解決する。
        let mut matrix: [[Cell; 80]; 24] = [
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
            [
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
                default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(), default_cell.clone(),
            ],
        ];

        // Set each position.
        for y in 0..matrix.len() {
            let row = &matrix[y];
            for x in 0..row.len() {
                matrix[y][x].position = (x as u8, y as u8);
            }
        }

        // Cache forground colors.
        let mut foreground_map: HashMap<ColorKind, String> = HashMap::new();
        foreground_map.insert(ColorKind::Black, format!("{}", color::Fg(color::Black)));
        foreground_map.insert(ColorKind::Blue, format!("{}", color::Fg(color::Blue)));
        foreground_map.insert(ColorKind::Cyan, format!("{}", color::Fg(color::Cyan)));
        foreground_map.insert(ColorKind::Green, format!("{}", color::Fg(color::Green)));
        foreground_map.insert(ColorKind::LightBlack, format!("{}", color::Fg(color::LightBlack)));
        foreground_map.insert(ColorKind::LightBlue, format!("{}", color::Fg(color::LightBlue)));
        foreground_map.insert(ColorKind::LightCyan, format!("{}", color::Fg(color::LightCyan)));
        foreground_map.insert(ColorKind::LightGreen, format!("{}", color::Fg(color::LightGreen)));
        foreground_map.insert(ColorKind::LightMagenta, format!("{}", color::Fg(color::LightMagenta)));
        foreground_map.insert(ColorKind::LightRed, format!("{}", color::Fg(color::LightRed)));
        foreground_map.insert(ColorKind::LightWhite, format!("{}", color::Fg(color::LightWhite)));
        foreground_map.insert(ColorKind::LightYellow, format!("{}", color::Fg(color::LightYellow)));
        foreground_map.insert(ColorKind::Magenta, format!("{}", color::Fg(color::Magenta)));
        foreground_map.insert(ColorKind::Red, format!("{}", color::Fg(color::Red)));
        foreground_map.insert(ColorKind::White, format!("{}", color::Fg(color::White)));
        foreground_map.insert(ColorKind::Yellow, format!("{}", color::Fg(color::Yellow)));

        // Cache background colors.
        let mut background_map: HashMap<ColorKind, String> = HashMap::new();
        background_map.insert(ColorKind::Black, format!("{}", color::Bg(color::Black)));
        background_map.insert(ColorKind::Blue, format!("{}", color::Bg(color::Blue)));
        background_map.insert(ColorKind::Cyan, format!("{}", color::Bg(color::Cyan)));
        background_map.insert(ColorKind::Green, format!("{}", color::Bg(color::Green)));
        background_map.insert(ColorKind::LightBlack, format!("{}", color::Bg(color::LightBlack)));
        background_map.insert(ColorKind::LightBlue, format!("{}", color::Bg(color::LightBlue)));
        background_map.insert(ColorKind::LightCyan, format!("{}", color::Bg(color::LightCyan)));
        background_map.insert(ColorKind::LightGreen, format!("{}", color::Bg(color::LightGreen)));
        background_map.insert(ColorKind::LightMagenta, format!("{}", color::Bg(color::LightMagenta)));
        background_map.insert(ColorKind::LightRed, format!("{}", color::Bg(color::LightRed)));
        background_map.insert(ColorKind::LightWhite, format!("{}", color::Bg(color::LightWhite)));
        background_map.insert(ColorKind::LightYellow, format!("{}", color::Bg(color::LightYellow)));
        background_map.insert(ColorKind::Magenta, format!("{}", color::Bg(color::Magenta)));
        background_map.insert(ColorKind::Red, format!("{}", color::Bg(color::Red)));
        background_map.insert(ColorKind::White, format!("{}", color::Bg(color::White)));
        background_map.insert(ColorKind::Yellow, format!("{}", color::Bg(color::Yellow)));
        
        Self {
            matrix,
            foreground_map,
            background_map,
        }
    }
    pub fn update(&mut self, screen_update: &ScreenUpdate) {
        // Map
        let map_xy = (2, 2);
        for (map_y, map_row) in screen_update.map.iter().enumerate() {
            for (map_x, map_element) in map_row.iter().enumerate() {
                let xy = (map_xy.0 + map_x, map_xy.1 + map_y);
                self.matrix[xy.1][xy.0].symbol = map_element.symbol;
                self.matrix[xy.1][xy.0].foreground = map_element.foreground.clone();
                self.matrix[xy.1][xy.0].background = map_element.background.clone();
            }
        }
    }
    /// Create the output of the specified line.
    ///
    /// If the foreground and the background are the same in consecutive cells, the output is omitted.
    pub fn create_one_line_output(&self, y: usize) -> String {
        let row = &self.matrix[y];
        let mut last_fg: &str = "";
        let mut last_bg: &str = "";
        let mut output: String = String::from("");
        for cell in row {
            let fg = self.foreground_map.get(&cell.foreground).unwrap();
            let bg = self.background_map.get(&cell.background).unwrap();
            if fg != last_fg {
                output = output + fg;
                last_fg = fg;
            }
            if bg != last_bg {
                output = output + bg;
                last_bg = bg;
            }
            output = output + &cell.symbol.to_string();
        }
        output
    }
    /// Reset the ANSI style after outputting these!
    pub fn create_output_as_lines(&self) -> Vec::<String> {
        self.matrix.iter().enumerate()
            .map(|(y, _)| self.create_one_line_output(y))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_instance() -> Screen {
        Screen::new()
    }

    mod tests_of_create_one_line_output {
        use super::*;

        #[test]
        fn it_can_compress_the_default_first_line() {
            let screen = create_test_instance();
            let first_cell = &screen.matrix[0][0];
            assert_eq!(
                screen.create_one_line_output(0),
                format!(
                    "{}{}{}",
                    screen.foreground_map.get(&first_cell.foreground).unwrap(),
                    screen.background_map.get(&first_cell.background).unwrap(),
                    String::from(" ").repeat(80),
                ),
            );
        }
        #[test]
        fn it_can_compress_the_output_when_it_changes_fg_and_bg_in_the_middle_of_the_line() {
            let mut screen = create_test_instance();
            screen.matrix[0][2] = Cell {
                foreground: ColorKind::Red,
                background: ColorKind::Blue,
                ..screen.matrix[0][2]
            };
            screen.matrix[0][3] = Cell {
                foreground: ColorKind::Red,
                background: ColorKind::Blue,
                ..screen.matrix[0][3]
            };
            let first_cell = &screen.matrix[0][0];
            let third_cell = &screen.matrix[0][2];
            assert_eq!(
                screen.create_one_line_output(0),
                format!(
                    "{}{}{}{}{}{}{}{}{}",
                    screen.foreground_map.get(&first_cell.foreground).unwrap(),
                    screen.background_map.get(&first_cell.background).unwrap(),
                    String::from(" ").repeat(2),
                    screen.foreground_map.get(&third_cell.foreground).unwrap(),
                    screen.background_map.get(&third_cell.background).unwrap(),
                    String::from(" ").repeat(2),
                    screen.foreground_map.get(&first_cell.foreground).unwrap(),
                    screen.background_map.get(&first_cell.background).unwrap(),
                    String::from(" ").repeat(76),
                ),
            );
        }
    }
}
