use crate::models::field_element::FieldElement;
use crate::models::field_object::FieldObject;
use crate::types::{FieldElementPosition, FieldObjectLocation, RectangleSize, XYCoordinates};
use crate::utils;

pub struct FieldSizeData {
    max_xy: XYCoordinates,
    size: RectangleSize,
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
            size: (width as u32, height as u32),
            max_xy: (width as i32 - 1, height as i32 - 1),
        }
    }
    pub fn get_rectangle_size(&self) -> RectangleSize {
        self.get_size_data().size
    }
    pub fn get_field_element(&self, position: &FieldElementPosition) -> &FieldElement {
        &self.matrix[position.1 as usize][position.0 as usize]
    }
    pub fn find_field_element_by_xy(&self, xy: &XYCoordinates) -> Option<&FieldElement> {
        let field_size_data = self.get_size_data();
        if xy.0 >= 0 && xy.0 <= field_size_data.max_xy.0 &&
            xy.1 >= 0 && xy.1 <= field_size_data.max_xy.1 {
            return Some(self.get_field_element(&(xy.0 as u32, xy.1 as u32)));
        }
        None
    }
    // TODO: field_objet の id を重複して発行しない。他の処理は id は重複してない前提にする。
    pub fn place_field_object(&mut self, position: &FieldElementPosition, field_object: FieldObject) {
        self.matrix[position.1 as usize][position.0 as usize].append_field_object(field_object);
    }
    pub fn move_field_object(&mut self, from: &FieldObjectLocation, to: &FieldElementPosition) {
        if &utils::xyi_to_xy(from) == to {
            panic!("Can not move to the same place.");
        }
        let from_field_element_pointer: *mut FieldElement = &mut self.matrix[from.1 as usize][from.0 as usize];
        let to_field_element_pointer: *mut FieldElement = &mut self.matrix[to.1 as usize][to.0 as usize];
        unsafe {
            let from_field_element = &mut *from_field_element_pointer;
            let to_field_element = &mut *to_field_element_pointer;
            from_field_element.move_field_object_to_another(&from.2, to_field_element);
        }
    }
    pub fn surround_with_walls(&mut self) {
        let field_size_data = self.get_size_data();
        for y in 0..field_size_data.size.1 {
            for x in 0..field_size_data.size.0 {
                if y == 0 || y == field_size_data.max_xy.1 as u32 || x == 0 || x == field_size_data.max_xy.0 as u32 {
                    // TODO: id の値が雑。
                    let id = format!("wall-{}-{}", x, y);
                    self.place_field_object(&(x, y), FieldObject::new_wall(id))
                }
            }
        }
    }
    // TODO: 引数は RectangleSize にする。
    pub fn new(width: usize, height: usize) -> Field {
        let mut matrix: Vec<Vec<FieldElement>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<FieldElement> = Vec::new();
            for x in 0..width {
                row.push(FieldElement {
                    position: (x as u32, y as u32),
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

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_of_find_field_element_by_xy {
        use super::*;

        mod when_it_returns_some {
            use super::*;

            struct TestCase {
                args: (XYCoordinates,),
                instance: Field,
                expected: FieldElementPosition,
            }

            #[test]
            fn it_works() {
                let table = vec![
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((0, 0),),
                        expected: ((0, 0)),
                    },
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((5, 0),),
                        expected: ((5, 0)),
                    },
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((0, 3),),
                        expected: ((0, 3)),
                    },
                ];
                for test_case in table {
                    assert_eq!(
                        test_case.instance.find_field_element_by_xy(&test_case.args.0).unwrap().get_position(),
                        test_case.expected,
                    );
                }
            }
        }

        mod when_it_returns_none {
            use super::*;

            struct TestCase {
                args: (XYCoordinates,),
                instance: Field,
            }

            #[test]
            fn it_works() {
                let table = vec![
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((-1, 0),),
                    },
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((0, -1),),
                    },
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((6, 0),),
                    },
                    TestCase {
                        instance: Field::new(6, 4),
                        args: ((0, 4),),
                    },
                ];
                for test_case in table {
                    assert_eq!(
                        test_case.instance.find_field_element_by_xy(&test_case.args.0).is_none(),
                        true,
                    );
                }
            }
        }
    }
}
