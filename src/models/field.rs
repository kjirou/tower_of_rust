use crate::models::field_element::FieldElement;
use crate::models::field_object::FieldObject;
use crate::types::{FieldElementPosition, FieldObjectPosition};
use crate::utils;

pub struct FieldSizeData {
    height: usize,
    max_x: usize,
    max_y: usize,
    width: usize,
}

#[derive(Debug)]
pub struct Field {
    matrix: Vec<Vec<FieldElement>>,
}

impl Field {
    pub fn get_size_data(&self) -> FieldSizeData {
        let height = self.matrix.len();
        if height == 0 {
            panic!("There are no rows in the field.");
        }
        let width = self.matrix[0].len();
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
        &self.matrix[xy.1][xy.0]
    }
    // TODO: field_objet の id を重複して発行しない。他の処理は id は重複してない前提にする。
    pub fn place_field_object(&mut self, xy: &FieldElementPosition, field_object: FieldObject) {
        self.matrix[xy.1][xy.0].append_field_object(field_object);
    }
    pub fn move_field_object(&mut self, from: &FieldObjectPosition, to: &FieldElementPosition) {
        if &utils::xyi_to_xy(from) == to {
            panic!("Can not move to the same place.");
        }
        let from_field_element_pointer: *mut FieldElement = &mut self.matrix[from.1][from.0];
        let to_field_element_pointer: *mut FieldElement = &mut self.matrix[to.1][to.0];
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
    pub fn new(width: usize, height: usize) -> Field {
        let mut matrix: Vec<Vec<FieldElement>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<FieldElement> = Vec::new();
            for x in 0..width {
                row.push(FieldElement {
                    y,
                    x,
                    field_objects: Vec::new(),
                });
            }
            matrix.push(row);
        }
        Field {
            matrix,
        }
    }
}
