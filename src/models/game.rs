use crate::models::field::FieldObjectPosition;

#[derive(Debug)]
pub struct Game {
    pub operation_target: Option<FieldObjectPosition>,
}
