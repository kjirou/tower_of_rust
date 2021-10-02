#[derive(Debug)]
pub enum DisplayKind {
    Hero,
    Wall,
}

#[derive(Debug)]
pub struct FieldObject {
    pub display_kind: DisplayKind,
    pub id: String,
    pub is_obstacle: bool,
}

impl FieldObject {
    pub fn new_hero(id: &str) -> Self {
        Self {
            id: id.to_string(),
            display_kind: DisplayKind::Hero,
            is_obstacle: true,
        }
    }
    pub fn new_wall(id: &str) -> Self {
        Self {
            id: id.to_string(),
            display_kind: DisplayKind::Wall,
            is_obstacle: true,
        }
    }
}
