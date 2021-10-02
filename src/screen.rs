use std::collections::HashMap;
use termion::color;

use crate::enums::{ColorKind};
use crate::screen_update::ScreenUpdate;
use crate::types::RectangleSize;

type ScreenCellPosition = (u8, u8);

#[derive(Clone)]
struct Cell {
    background: ColorKind,
    foreground: ColorKind,
    /// (x, y)
    position: ScreenCellPosition,
    symbol: char,
}

pub struct WriteTextParameters {
    auto_line_break: bool,
    background: Option<ColorKind>,
    foreground: Option<ColorKind>,
}
impl Default for WriteTextParameters {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            auto_line_break: false,
        }
    }
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
    pub fn get_size(&self) -> RectangleSize {
        (self.matrix[0].len() as u32, self.matrix.len() as u32)
    }
    /// Write the text into the rectangle.
    pub fn write_text(&mut self, position: &ScreenCellPosition, size: &RectangleSize, text: &str, params: &WriteTextParameters) {
        let chars: Vec<char> = text.chars().collect();
        let bottom_right_position: (u8, u8) = (position.0 + size.0 as u8 - 1, position.1 + size.1 as u8 - 1);
        let screen_size = self.get_size();
        if bottom_right_position.0 >= screen_size.0 as u8 || bottom_right_position.1 >= screen_size.1 as u8 {
            panic!("The text area is out of the screen.");
        }
        let mut pointer: (usize, usize) = (position.0 as usize, position.1 as usize);
        for ch in chars {
            if pointer.1 > bottom_right_position.1 as usize {
                break;
            }
            if ch == '\n' {
                pointer.0 = 0;
                pointer.1 += 1;
                continue;
            }
            if pointer.0 <= bottom_right_position.0 as usize {
                self.matrix[pointer.1][pointer.0].symbol = ch;
            }
            if params.foreground.is_some() {
                self.matrix[pointer.1][pointer.0].foreground = params.foreground.clone().unwrap();
            }
            if params.background.is_some() {
                self.matrix[pointer.1][pointer.0].background = params.background.clone().unwrap();
            }
            if pointer.0 >= bottom_right_position.0 as usize && params.auto_line_break {
                pointer.0 = 0;
                pointer.1 += 1;
            } else {
                pointer.0 += 1;
            }
        }
    }
    pub fn update(&mut self, screen_update: &ScreenUpdate) {
        // Map
        let map_position = (2, 2);
        for (map_y, map_row) in screen_update.map.iter().enumerate() {
            for (map_x, map_element) in map_row.iter().enumerate() {
                let position = (map_position.0 + map_x, map_position.1 + map_y);
                self.matrix[position.1][position.0].symbol = map_element.symbol;
                self.matrix[position.1][position.0].foreground = map_element.foreground.clone();
                self.matrix[position.1][position.0].background = map_element.background.clone();
            }
        }

        // Debug prints
        let debug_prints = format!(
            "{}",
            screen_update.last_key_input,
        );
        self.write_text(&(70, 0), &(10, 3), &debug_prints, &WriteTextParameters {
            auto_line_break: false,
            ..Default::default()
        });
    }
    pub fn dump_as_text(&self, position: &ScreenCellPosition, size: &RectangleSize) -> String {
        let mut text: String = String::from("");
        for y in (position.1)..(position.1 + size.1 as u8) {
            for x in (position.1)..(position.0 + size.0 as u8) {
                text += &self.matrix[y as usize][x as usize].symbol.to_string();
            }
            if y != position.1 + size.1 as u8 - 1 {
                text += &"\n";
            }
        }
        text
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
    /// Create the output as lines of string.
    /// 
    /// You should reset the ANSI style after outputting these.
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

    mod tests_of_dump_as_text {
        use super::*;

        #[test]
        fn it_works() {
            let mut screen = Screen {
                ..create_test_instance()
            };
            screen.matrix[1][2].symbol = 'a';
            screen.matrix[1][3].symbol = 'b';
            screen.matrix[2][3].symbol = 'c';
            screen.matrix[2][4].symbol = 'd';
            assert_eq!(
                screen.dump_as_text(&(0, 0), &(5, 4)),
                vec![
                    "     ",
                    "  ab ",
                    "   cd",
                    "     ",
                ].join("\n"),
            );
        }
    }

    mod tests_of_write_text {
        use super::*;

        #[test]
        #[should_panic(expected = " out of the screen")]
        fn it_should_panic_when_the_horizontal_extent_of_the_text_is_out_of_the_screen() {
            let mut instance = create_test_instance();
            let screen_size = instance.get_size();
            instance.write_text(&(1, 1), &(screen_size.0, 1), "", &Default::default());
        }
        #[test]
        #[should_panic(expected = " out of the screen")]
        fn it_should_panic_when_the_vertical_extent_of_the_text_is_out_of_the_screen() {
            let mut instance = create_test_instance();
            let screen_size = instance.get_size();
            instance.write_text(&(1, 1), &(1, screen_size.1), "", &Default::default());
        }
        #[test]
        fn it_should_do_nothing_when_there_are_zero_chars() {
            let mut instance = create_test_instance();
            let before_dump = instance.dump_as_text(&(0, 0), &(80, 24));
            instance.write_text(&(0, 0), &(80, 24), "", &Default::default());
            let after_dump = instance.dump_as_text(&(0, 0), &(80, 24));
            assert_eq!(before_dump, after_dump);
        }
        #[test]
        fn it_can_write_a_single_line() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(80, 24), "abc", &Default::default());
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(4, 2)),
                vec![
                    "abc ",
                    "    ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_can_set_any_starting_point() {
            let mut instance = create_test_instance();
            instance.write_text(&(2, 1), &(1, 1), "a", &Default::default());
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(4, 3)),
                vec![
                    "    ",
                    "  a ",
                    "    ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_can_break_lines_with_a_line_feed_character() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(80, 24), "a\nbc\ndef", &Default::default());
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(4, 4)),
                vec![
                    "a   ",
                    "bc  ",
                    "def ",
                    "    ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_should_not_break_lines_when_the_text_reaches_the_right_edge() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(3, 24), "12345\n67890", &Default::default());
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(4, 3)),
                vec![
                    "123 ",
                    "678 ",
                    "    ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_should_break_lines_when_the_text_reaches_the_right_edge_but_the_auto_line_break_is_true() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(3, 24), "1234567", &WriteTextParameters {
                auto_line_break: true,
                ..Default::default()
            });
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(4, 4)),
                vec![
                    "123 ",
                    "456 ",
                    "7   ",
                    "    ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_should_ignore_when_the_number_of_lines_is_exceeded() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(3, 2), "1\n2\n3", &Default::default());
            assert_eq!(
                instance.dump_as_text(&(0, 0), &(2, 3)),
                vec![
                    "1 ",
                    "2 ",
                    "  ",
                ].join("\n"),
            );
        }
        #[test]
        fn it_can_set_colors() {
            let mut instance = create_test_instance();
            instance.write_text(&(0, 0), &(80, 24), "1", &WriteTextParameters {
                foreground: Some(ColorKind::Red),
                background: Some(ColorKind::Blue),
                ..Default::default()
            });
            assert_eq!(instance.matrix[0][0].foreground, ColorKind::Red);
            assert_eq!(instance.matrix[0][0].background, ColorKind::Blue);
        }
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
