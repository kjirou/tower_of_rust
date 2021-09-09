use crate::models::field_object::DisplayKind;
use crate::models::field_object::FieldObject;
use crate::types::FieldElementPosition;

#[derive(Debug, Default)]
pub struct FieldElement {
    pub field_objects: Vec<FieldObject>,
    pub x: usize,
    pub y: usize,
}

impl FieldElement {
    pub fn get_position(&self) -> FieldElementPosition {
        (self.x, self.y)
    }
    pub fn is_impassable(&self) -> bool {
        self.field_objects.iter().any(|e| e.is_obstacle)
    }
    // TODO: foreground と　background を生成するための情報も必要。
    // TODO: 中間に Props を作り、screen と models の両面へ DisplayKind 的な型を反映する。
    pub fn get_display(&self) -> char {
        if self.field_objects.len() == 0 {
            return ' '
        }
        // TODO: 先頭の FieldObject が常に描画対象になるかは要検討。
        let first = &self.field_objects[0];
        match first.display_kind {
            DisplayKind::Hero => '@',
            DisplayKind::Wall => '#',
        }
    }
    pub fn append_field_object(&mut self, field_object: FieldObject) {
        self.field_objects.push(field_object);
    }
    fn remove_field_object(&mut self, id: &str) -> FieldObject {
        let index = self.field_objects.iter().position(|e| e.id == id);
        match index {
            Some(index) => {
                return self.field_objects.remove(index);
            },
            None => panic!("Can not find the id."),
        };
    }
    pub fn move_field_object_to_another(&mut self, id: &str, to: &mut FieldElement) {
        if to.is_impassable() {
            panic!("The field element {:?} is impassable.", to.get_position());
        }
        let moved = self.remove_field_object(id);
        to.append_field_object(moved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_not_obstacle(id: String) -> FieldObject {
        FieldObject {
            is_obstacle: false,
            ..FieldObject::new_wall(id)
        }
    }

    fn create_obstacle(id: String) -> FieldObject {
        FieldObject::new_wall(id)
    }

    mod tests_of_is_impassable {
        use super::*;

        #[test]
        fn it_returns_false_when_field_objects_are_empty() {
            let field_element = FieldElement { field_objects: vec![], ..Default::default() };
            assert_eq!(field_element.is_impassable(), false);
        }
        #[test]
        fn it_returns_true_when_it_have_an_obstacle() {
            let field_element = FieldElement {
                field_objects: vec![
                    create_obstacle(String::from("a")),
                ],
                ..Default::default()
            };
            assert_eq!(field_element.is_impassable(), true);
        }
        #[test]
        fn it_returns_false_when_it_have_a_not_obstacle() {
            let field_element = FieldElement {
                field_objects: vec![
                    create_not_obstacle(String::from("a")),
                ],
                ..Default::default()
            };
            assert_eq!(field_element.is_impassable(), false);
        }
    }

    mod tests_of_move_field_object_to_another {
        use super::*;

        #[test]
        fn it_can_move_a_field_object_to_the_passable_field_element() {
            let mut from = FieldElement {
                field_objects: vec![
                    create_obstacle(String::from("a")),
                ],
                ..Default::default()
            };
            let mut to = FieldElement {
                field_objects: vec![
                    create_not_obstacle(String::from("b")),
                ],
                ..Default::default()
            };
            from.move_field_object_to_another("a", &mut to);
            assert_eq!(from.field_objects.len(), 0);
            assert_eq!(to.field_objects.len(), 2);
        }

        #[test]
        #[should_panic(expected = " is impassable")]
        fn it_panics_when_the_destination_field_element_is_impassable() {
            let mut from = FieldElement {
                field_objects: vec![
                    create_obstacle(String::from("a")),
                ],
                ..Default::default()
            };
            let mut to = FieldElement {
                field_objects: vec![
                    create_obstacle(String::from("b")),
                ],
                ..Default::default()
            };
            from.move_field_object_to_another("a", &mut to);
        }
    }
}
