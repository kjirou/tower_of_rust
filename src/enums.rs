use std::fmt;

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
impl fmt::Display for FourDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Up => "Up",
            Self::Right => "Right",
            Self::Down => "Down",
            Self::Left => "Left",
        };
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests_of_four_direction {
    use super::*;

    #[test]
    fn it_can_display() {
        assert_eq!(FourDirection::Up.to_string(), String::from("Up"));
        assert_eq!(FourDirection::Right.to_string(), String::from("Right"));
        assert_eq!(FourDirection::Down.to_string(), String::from("Down"));
        assert_eq!(FourDirection::Left.to_string(), String::from("Left"));
    }
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
