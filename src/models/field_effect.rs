#[derive(Debug)]
pub struct FieldEffect {
    /// It is unique throughout the application.
    id: String,
    /// It is the number of frames since it was created. Count from 0.
    number_of_frames: u64,
}
impl FieldEffect {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            number_of_frames: 0,
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
