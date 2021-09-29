#[derive(Debug, PartialEq)]
pub enum CustomErrorKind {
    CoordinateIsOutsideOfPosition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FourDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ColorKind {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow,
}
