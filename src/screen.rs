#[derive(Clone)]
pub struct Cell {
    // TODO: いくつかの ANCI escape code の列挙を設定する。
    pub background: String,
    pub foreground: String,
    pub symbol: char,
    pub x: u8,
    pub y: u8,
}

pub struct Screen {
    pub matrix: [[Cell; 80]; 24],
}
