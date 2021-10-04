use crate::enums::FourDirection;
use crate::mediators::*;
use crate::models::field::Field;
use crate::models::game::Game;
use crate::types::FieldElementPosition;

fn change_placement_of_operation_target(field: &mut Field, game: &mut Game, to: &FieldElementPosition) {
    match &game.operation_target_location {
        Some(operation_target_location) => {
            field.move_field_object(operation_target_location, to);
            game.operation_target_location = Some(((to.0, to.1), operation_target_location.1.clone()));
        },
        None => {
            panic!("There is no operation target.");
        },
    };
}

pub fn move_operation_target_by_consuming_its_movement_power(
    field: &mut Field, game: &mut Game, to: &FieldElementPosition, direction: &FourDirection,
) {
    change_placement_of_operation_target(field, game, to);
    let operation_target = get_operation_target_mut(field, game);
    operation_target.consume_movement_power_for_step();
    operation_target.direction = direction.clone();
}
