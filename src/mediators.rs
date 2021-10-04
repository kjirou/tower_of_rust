use crate::models::field::Field;
use crate::models::field_object::FieldObject;
use crate::models::game::Game;

pub fn find_operation_target<'a>(field: &'a Field, game: &'a Game) -> Option<&'a FieldObject> {
    if let Some(operation_target_location) = &game.operation_target_location {
        return field.find_field_object(operation_target_location);
    }
    None
}

pub fn get_operation_target<'a>(field: &'a Field, game: &'a Game) -> &'a FieldObject {
    find_operation_target(field, game).unwrap()
}

pub fn get_operation_target_mut<'a>(field: &'a mut Field, game: &'a Game) -> &'a mut FieldObject {
    field.find_field_object_mut(&game.operation_target_location.clone().unwrap()).unwrap()
}
