use crate::enums::FourDirection;
use crate::errors::CoordinateIsOutsideOfPositionError;
use crate::models::field::{Field, xyi_to_xy};
use crate::models::game::Game;
use crate::types::FieldElementPosition;
use crate::unit_of_works::*;

// TODO: utils みたいなのへ移動する。
fn translate_coordinate(start: &(i32, i32), vector: &(i32, i32)) -> (i32, i32) {
    (start.0 + vector.0, start.1 + vector.1)
}

// TODO: 最大のxyを指定できるようにする。
fn translate_position_by_direction(
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

// TODO: enum の引数って通常は参照で受け取るべき？
pub fn move_hero(field: &mut Field, game: &mut Game, direction: FourDirection) {
    match &game.operation_target {
        Some(operation_target) => {
            match translate_position_by_direction(&xyi_to_xy(operation_target), direction) {
                Ok(position) => {
                    move_operation_target(field, game, &position);
                    // TODO: 障害物との衝突判定。
                },
                Err(_) => {},
            };
        },
        None => {},
    };
}
