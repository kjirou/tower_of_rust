use termion::event::Key;

use crate::types::FieldObjectLocation;

#[derive(Debug)]
pub struct Game {
    pub last_key_input: Option<Key>,
    pub number_of_frames: u64,
    pub operation_target: Option<FieldObjectLocation>,
}
