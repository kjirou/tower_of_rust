use crate::models::field::Field;
use crate::models::game::Game;
use crate::types::FieldElementPosition;

pub fn move_operation_target(game: &Game, field: &mut Field, to: &FieldElementPosition) {
    match &game.operation_target {
        Some(operation_target) => field.move_field_object(operation_target, to),
        None => {
            panic!("There is no operation target.");
        },
    };
}
