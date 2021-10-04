use crate::enums::FourDirection;
use crate::id_generator::IdGenerator;
use crate::models::field::Field;
use crate::models::field_effect::FieldEffect;
use crate::models::game::Game;
use crate::unit_of_works::*;
use crate::utils::{translate_position_by_direction};

pub fn moves_one_step(field: &mut Field, game: &mut Game, direction: &FourDirection) {
    if let Some(operation_target_location) = &game.operation_target_location {
        if let Ok(position) = translate_position_by_direction(&field.get_rectangle_size(), &operation_target_location.0, direction) {
            let destination = field.get_field_element(&position);
            if !destination.is_impassable() {
                let operation_target = get_operation_target(field, game);
                if operation_target.can_step() {
                    move_operation_target_by_consuming_its_movement_power(field, game, &position);
                }
            }
        }
    }
}

// TODO: 向いている方向へ効果を配置する。
pub fn makes_attack(id_generator: &mut IdGenerator, field: &mut Field, game: &Game) {
    if let Some(operation_target_location) = &game.operation_target_location {
        if let Ok(position) = translate_position_by_direction(&field.get_rectangle_size(), &operation_target_location.0, &FourDirection::Up) {
            field.place_field_effect(&position, FieldEffect::new(&id_generator.generate_for_field_effect()));
        }
    }
}
