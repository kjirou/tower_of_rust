#[derive(Debug)]
pub enum DisplayKind {
    Hero,
    Wall,
}

#[derive(Debug)]
pub struct FieldObject {
    pub display_kind: DisplayKind,
    is_obstacle: bool,
}

impl FieldObject {
    pub fn new_hero() -> FieldObject {
        FieldObject {
            display_kind: DisplayKind::Hero,
            is_obstacle: true,
        }
    }
    pub fn new_wall() -> FieldObject {
        FieldObject {
            display_kind: DisplayKind::Wall,
            is_obstacle: true,
        }
    }
}
