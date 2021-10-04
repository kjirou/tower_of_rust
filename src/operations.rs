use crate::enums::FourDirection;
use crate::id_generator::IdGenerator;
use crate::mediators::{find_operation_target_mut, get_operation_target};
use crate::models::field::Field;
use crate::models::field_effect::FieldEffect;
use crate::models::game::Game;
use crate::unit_of_works::*;
use crate::utils::{translate_position_by_direction};

pub fn changes_direction(field: &mut Field, game: &mut Game, direction: &FourDirection) {
    if let Some(operation_target) = find_operation_target_mut(field, game) {
        operation_target.direction = direction.clone();
    }
}

pub fn moves_one_step(field: &mut Field, game: &mut Game, direction: &FourDirection) {
    if let Some(operation_target_location) = &game.operation_target_location {
        if let Ok(position) = translate_position_by_direction(&field.get_rectangle_size(), &operation_target_location.0, direction) {
            let destination = field.get_field_element(&position);
            if !destination.is_impassable() {
                let operation_target = get_operation_target(field, game);
                if operation_target.can_step() {
                    move_operation_target_by_consuming_its_movement_power(field, game, &position, direction);
                }
            }
        }
    }
}

pub fn makes_attack(id_generator: &mut IdGenerator, field: &mut Field, game: &Game) {
    if let Some(operation_target_location) = &game.operation_target_location {
        let operation_target = field.find_field_object(operation_target_location).unwrap();
        if let Ok(position) = translate_position_by_direction(&field.get_rectangle_size(), &operation_target_location.0, &operation_target.direction) {
            field.place_field_effect(&position, FieldEffect::new(&id_generator.generate_for_field_effect()));
        }
    }
}
