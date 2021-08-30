use crate::models::field::Field;
use crate::models::game::Game;
use crate::types::FieldElementPosition;

pub fn move_operation_target(field: &mut Field, game: &mut Game, to: &FieldElementPosition) {
    match &game.operation_target {
        Some(operation_target) => {
            field.move_field_object(operation_target, to);
            game.operation_target = Some((to.0, to.1, operation_target.2.clone()));
        },
        None => {
            panic!("There is no operation target.");
        },
    };
}
