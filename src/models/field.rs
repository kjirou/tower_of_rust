#[derive(Debug)]
pub struct FieldElement {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Field {
    pub matrix: Vec<Vec<FieldElement>>,
}

impl Field {
    pub fn new(width: u32, height: u32) -> Field {
        let mut matrix: Vec<Vec<FieldElement>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<FieldElement> = Vec::new();
            for x in 0..width {
                row.push(FieldElement {
                    y,
                    x,
                });
            }
            matrix.push(row);
        }
        Field {
            matrix,
        }
    }
}
