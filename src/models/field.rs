use crate::models::field_object::DisplayKind;
use crate::models::field_object::FieldObject;

#[derive(Debug)]
pub struct FieldElement {
    // TODO: FieldObject リストを直接走査するために別にリストにするかもしれないので、そうなると可変参照のベクタになりそう。
    field_objects: Vec<FieldObject>,
    x: usize,
    y: usize,
}

impl FieldElement {
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

#[derive(Debug)]
pub struct Field {
    pub matrix: Vec<Vec<FieldElement>>,
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
    pub fn place_field_object(&mut self, xy: (usize, usize), field_object: FieldObject) {
        self.matrix[xy.1][xy.0].field_objects.push(field_object);
    }
    pub fn surround_with_walls(&mut self) {
        let size = self.get_size_data();
        for y in 0..size.height {
            for x in 0..size.width {
                if y == 0 || y == size.max_y || x == 0 || x == size.max_x {
                    self.matrix[y][x].field_objects.push(FieldObject::new_wall());
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
