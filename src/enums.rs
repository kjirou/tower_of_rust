#[derive(Debug, PartialEq)]
pub enum CustomErrorKind {
    CoordinateIsOutsideOfPosition,
}

#[derive(Debug)]
pub enum FourDirection {
    Up,
    Right,
    Down,
    Left,
}
