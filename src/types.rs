/// (width, height)
pub type RectangleSize = (u32, u32);

/// (x, y)
pub type XYCoordinates = (i32, i32);

/// (x, y)
pub type XYVector = (i32, i32);

/// A (x, y) coordinates of a `FieldElement`.
pub type FieldElementPosition = (usize, usize);

/// A location of a `FieldObject`.
/// 
/// It consists of (x, y, id_of_field_object).  
/// The (x, y) part is equal to a `FieldElementPosition`.
// TODO: 他の type の型を使えないか。
pub type FieldObjectLocation = (usize, usize, String);
