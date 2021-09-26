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
