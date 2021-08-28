use crate::models::field_object::DisplayKind;
use crate::models::field_object::FieldObject;

#[derive(Debug)]
pub struct FieldElement {
    field_objects: Vec<FieldObject>,
    x: usize,
    y: usize,
}

impl FieldElement {
    fn append_field_object(&mut self, field_object: FieldObject) {
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
    fn move_field_object_to_another(&mut self, id: &str, to: &mut FieldElement) {
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

pub struct FieldSizeData {
    height: usize,
    max_x: usize,
    max_y: usize,
    width: usize,
}

// (x, y)
type FieldElementPosition = (usize, usize);

// (x, y, field_object.id)
type FieldObjectPosition = (usize, usize, String);

pub fn xyi_to_xy(xyi: &FieldObjectPosition) -> FieldElementPosition {
    (xyi.1, xyi.0)
}

#[derive(Debug)]
pub struct FieldMatrix {
    data: Vec<Vec<FieldElement>>,
}

impl FieldMatrix {
    pub fn get_size_data(&self) -> FieldSizeData {
        let height = self.data.len();
        if height == 0 {
            panic!("There are no rows in the field.");
        }
        let width = self.data[0].len();
        if width == 0 {
            panic!("There are no columns in the field.");
        }
        FieldSizeData {
            height,
            width,
            max_x: width - 1,
            max_y: height - 1,
        }
    }
    pub fn get_field_element(&self, xy: &FieldElementPosition) -> &FieldElement {
        &self.data[xy.1][xy.0]
    }
    // TODO: field_objet の id を重複して発行しない。他の処理は id は重複してない前提にする。
    pub fn place_field_object(&mut self, xy: &FieldElementPosition, field_object: FieldObject) {
        self.data[xy.1][xy.0].append_field_object(field_object);
    }
    pub fn move_field_object(&mut self, from: &FieldObjectPosition, to: &FieldElementPosition) {
        if &xyi_to_xy(from) == to {
            panic!("Can not move to the same place.");
        }
        let from_field_element_pointer: *mut FieldElement = &mut self.data[from.1][from.0];
        let to_field_element_pointer: *mut FieldElement = &mut self.data[to.1][to.0];
        unsafe {
            let from_field_element = &mut *from_field_element_pointer;
            let to_field_element = &mut *to_field_element_pointer;
            from_field_element.move_field_object_to_another(&from.2, to_field_element);
        }
    }
    pub fn surround_with_walls(&mut self) {
        let size = self.get_size_data();
        for y in 0..size.height {
            for x in 0..size.width {
                if y == 0 || y == size.max_y || x == 0 || x == size.max_x {
                    // TODO: id の値が雑。
                    let id = format!("wall-{}-{}", x, y);
                    self.place_field_object(&(x, y), FieldObject::new_wall(id))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub matrix: FieldMatrix,
    pub operation_target: Option<FieldObjectPosition>,
}

impl Field {
    pub fn move_operation_target(&mut self, to: &FieldElementPosition) {
        match &self.operation_target {
            Some(operation_target) => self.matrix.move_field_object(operation_target, to),
            None => {
                panic!("There is no operation target.");
            },
        };
    }
    pub fn new(width: usize, height: usize) -> Field {
        let mut matrix_data: Vec<Vec<FieldElement>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<FieldElement> = Vec::new();
            for x in 0..width {
                row.push(FieldElement {
                    y,
                    x,
                    field_objects: Vec::new(),
                });
            }
            matrix_data.push(row);
        }
        Field {
            matrix: FieldMatrix {
                data: matrix_data,
            },
            operation_target: None,
        }
    }
}
