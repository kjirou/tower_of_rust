use crate::models::field_element::FieldElement;
use crate::models::field_object::FieldObject;
use crate::types::{FieldElementPosition, FieldObjectLocation, RectangleSize, XYCoordinates};
use crate::utils::dungeon_generator::{Dungeon, DungeonCellKind};

pub struct FieldSizeData {
    max_xy: XYCoordinates,
    size: RectangleSize,
}

#[derive(Debug)]
pub struct Field {
    matrix: Vec<Vec<FieldElement>>,
}
impl Field {
    pub fn new(size: &RectangleSize) -> Self {
        let mut matrix: Vec<Vec<FieldElement>> = Vec::new();
        for y in 0..size.1 {
            let mut row: Vec<FieldElement> = Vec::new();
            for x in 0..size.0 {
                row.push(FieldElement::new(&(x, y)));
            }
            matrix.push(row);
        }
        Self {
            matrix,
        }
    }
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
    pub fn get_field_element_mut(&mut self, position: &FieldElementPosition) -> &mut FieldElement {
        &mut self.matrix[position.1 as usize][position.0 as usize]
    }
    pub fn find_field_element_by_xy(&self, xy: &XYCoordinates) -> Option<&FieldElement> {
        let field_size_data = self.get_size_data();
        if xy.0 >= 0 && xy.0 <= field_size_data.max_xy.0 &&
            xy.1 >= 0 && xy.1 <= field_size_data.max_xy.1 {
            return Some(self.get_field_element(&(xy.0 as u32, xy.1 as u32)));
        }
        None
    }
    pub fn find_field_object(&self, location: &FieldObjectLocation) -> Option<&FieldObject> {
        self.get_field_element(&location.0).find_field_object(&location.1)
    }
    pub fn find_field_object_mut(&mut self, location: &FieldObjectLocation) -> Option<&mut FieldObject> {
        self.get_field_element_mut(&location.0).find_field_object_mut(&location.1)
    }
    // TODO: field_objet の id を重複して発行しない。他の処理は id は重複してない前提にする。
    pub fn place_field_object(&mut self, position: &FieldElementPosition, field_object: FieldObject) {
        self.get_field_element_mut(position).append_field_object(field_object);
    }
    pub fn move_field_object(&mut self, from: &FieldObjectLocation, to: &FieldElementPosition) {
        if &from.0 == to {
            panic!("Can not move to the same place.");
        }
        let from_field_element_pointer: *mut FieldElement = &mut self.matrix[from.0.1 as usize][from.0.0 as usize];
        let to_field_element_pointer: *mut FieldElement = &mut self.matrix[to.1 as usize][to.0 as usize];
        unsafe {
            let from_field_element = &mut *from_field_element_pointer;
            let to_field_element = &mut *to_field_element_pointer;
            from_field_element.move_field_object_to_another(&from.1, to_field_element);
        }
    }
    pub fn import_dungeon(&mut self, dungeon: &Dungeon) {
        let size_data = self.get_size_data();
        if size_data.size != dungeon.get_size() {
            panic!("The sizes of the field and the dungeon do not match.");
        }
        for y in 0..size_data.size.1 {
            for x in 0..size_data.size.0 {
                let yu = y as usize;
                let xu = x as usize;
                match dungeon.matrix[yu][xu].kind {
                    DungeonCellKind::Wall | DungeonCellKind::Blank => {
                        // TODO: id の生成手順を管理する。
                        let wall_id = format!("wall-{}-{}", x, y);
                        self.place_field_object(&(x, y), FieldObject::new_wall(&wall_id));
                    },
                    _ => {},
                };
            }
        }
    }
    pub fn perform_state_changes_over_time(&mut self) {
        let size = self.get_rectangle_size();
        for y in 0..size.1 as usize {
            for x in 0..size.0 as usize {
                for field_object in &mut self.matrix[y][x].field_objects {
                    if field_object.can_move() {
                        field_object.accumulate_movement_power();
                    }
                }
            }
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
                        instance: Field::new(&(6, 4)),
                        args: ((0, 0),),
                        expected: ((0, 0)),
                    },
                    TestCase {
                        instance: Field::new(&(6, 4)),
                        args: ((5, 0),),
                        expected: ((5, 0)),
                    },
                    TestCase {
                        instance: Field::new(&(6, 4)),
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
                        instance: Field::new(&(6, 4)),
                        args: ((-1, 0),),
                    },
                    TestCase {
                        instance: Field::new(&(6, 4)),
                        args: ((0, -1),),
                    },
                    TestCase {
                        instance: Field::new(&(6, 4)),
                        args: ((6, 0),),
                    },
                    TestCase {
                        instance: Field::new(&(6, 4)),
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

    mod tests_of_perform_state_changes_over_time {
        use super::*;

        fn create_test_movable_field_object(id: &str) -> FieldObject {
            FieldObject {
                mobility: 1,
                ..FieldObject::new_hero(id)
            }
        }

        #[test]
        fn it_works() {
            let mut field = Field::new(&(2, 1));
            let a = create_test_movable_field_object("a");
            let b = create_test_movable_field_object("b");
            let c = create_test_movable_field_object("c");
            field.place_field_object(&(0, 0), a);
            field.place_field_object(&(0, 0), b);
            field.place_field_object(&(1, 0), c);
            assert_eq!(
                field.find_field_object(&((0, 0), String::from("a"))).unwrap().movement_power,
                0,
            );
            assert_eq!(
                field.find_field_object(&((0, 0), String::from("b"))).unwrap().movement_power,
                0,
            );
            assert_eq!(
                field.find_field_object(&((1, 0), String::from("c"))).unwrap().movement_power,
                0,
            );
            field.perform_state_changes_over_time();
            assert_eq!(
                field.find_field_object(&((0, 0), String::from("a"))).unwrap().movement_power,
                1,
            );
            assert_eq!(
                field.find_field_object(&((0, 0), String::from("b"))).unwrap().movement_power,
                1,
            );
            assert_eq!(
                field.find_field_object(&((1, 0), String::from("c"))).unwrap().movement_power,
                1,
            );
        }
    }
}
