use crate::models::field_object::DisplayKind;
use crate::models::field_object::FieldObject;

#[derive(Debug)]
pub struct FieldElement {
    pub field_objects: Vec<FieldObject>,
    pub x: usize,
    pub y: usize,
}

impl FieldElement {
    pub fn append_field_object(&mut self, field_object: FieldObject) {
        self.field_objects.push(field_object);
    }
    // fn find_field_object(&self, id: &str) -> Option<&FieldObject> {
    //     self.field_objects.iter().find(|&e| e.id == id)
    // }
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
        // TODO: 移動先に障害物があるときに panic! にする。
        let moved = self.remove_field_object(id);
        to.append_field_object(moved);
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
}
