#[derive(Debug)]
pub struct FieldEffect {
    /// It is unique throughout the application.
    id: String,
    // TODO: おそらく衝突したことがある FieldObject の情報を格納する必要がある。
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
