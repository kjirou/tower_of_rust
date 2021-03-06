#[derive(Debug)]
pub struct FieldEffect {
    /// It is unique throughout the application.
    id: String,
}
impl FieldEffect {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
