use crate::enums::FourDirection;
use crate::models::field::Field;
use crate::models::game::Game;
use crate::unit_of_works::*;
use crate::utils::{translate_position_by_direction};

pub fn move_hero(field: &mut Field, game: &mut Game, direction: &FourDirection) {
    if let Some(operation_target) = &game.operation_target {
        if let Ok(position) = translate_position_by_direction(&field.get_rectangle_size(), &operation_target.0, direction) {
            let destination = field.get_field_element(&position);
            if !destination.is_impassable() {
                let operation_target = get_operation_target(field, game);
                if operation_target.can_step() {
                    move_operation_target_for_one_step(field, game, &position);
                }
            }
        }
    }
}
