use crate::enums::FourDirection;
use crate::errors::CoordinateIsOutsideOfPositionError;
use crate::types::{FieldElementPosition, FieldObjectPosition};

pub fn xyi_to_xy(xyi: &FieldObjectPosition) -> FieldElementPosition {
    (xyi.0, xyi.1)
}

pub fn translate_coordinate(start: &(i32, i32), vector: &(i32, i32)) -> (i32, i32) {
    (start.0 + vector.0, start.1 + vector.1)
}

// TODO: 最大のxyを指定できるようにする。
pub fn translate_position_by_direction(
    start: &FieldElementPosition, direction: FourDirection
) -> Result<FieldElementPosition, CoordinateIsOutsideOfPositionError> {
    let vector: (i32, i32) = match direction {
        FourDirection::Up => (0, -1),
        FourDirection::Right => (1, 0),
        FourDirection::Down => (0, 1),
        FourDirection::Left => (-1, 0),
    };
    let moved = translate_coordinate(&(start.0 as i32, start.1 as i32), &vector);
    if moved.0 < 0 || moved.1 < 0 {
        return Err(CoordinateIsOutsideOfPositionError);
    }
    Ok((moved.0 as usize, moved.1 as usize))
}
