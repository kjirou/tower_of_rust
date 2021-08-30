use crate::enums::FourDirection;
use crate::models::field::Field;
use crate::models::game::Game;
use crate::unit_of_works::*;
use crate::utils::*;

pub fn advance_only_time() {
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
