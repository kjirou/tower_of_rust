use crate::models::field::Field;
use crate::models::field_object::FieldObject;
use crate::models::game::Game;
use crate::types::FieldElementPosition;

pub fn get_operation_target<'a>(field: &'a Field, game: &'a Game) -> &'a FieldObject {
    field.find_field_object(&game.operation_target_location.clone().unwrap()).unwrap()
}

fn get_operation_target_mut<'a>(field: &'a mut Field, game: &'a Game) -> &'a mut FieldObject {
    field.find_field_object_mut(&game.operation_target_location.clone().unwrap()).unwrap()
}

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

pub fn move_operation_target_for_one_step(field: &mut Field, game: &mut Game, to: &FieldElementPosition) {
    change_placement_of_operation_target(field, game, to);
    get_operation_target_mut(field, game).consume_movement_power_for_step();
}
