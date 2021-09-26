use crate::enums::FourDirection;
use crate::models::field::Field;
use crate::models::game::Game;
use crate::unit_of_works::*;
use crate::utils::*;

pub fn advance_only_time() {
}

pub fn move_hero(field: &mut Field, game: &mut Game, direction: &FourDirection) {
    match &game.operation_target {
        Some(operation_target) => {
            match translate_position_by_direction(&field.get_rectangle_size(), &operation_target.0, direction) {
                Ok(position) => {
                    let destination = field.get_field_element(&position);
                    if !destination.is_impassable() {
                        move_operation_target(field, game, &position);
                    }
                },
                Err(_) => {},
            };
        },
        None => {},
    };
}
