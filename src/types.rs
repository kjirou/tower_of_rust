// (width, height)
pub type RectangleSize = (u32, u32);

// (x, y)
pub type FieldElementPosition = (usize, usize);

// (FieldElementPosition.0, FieldElementPosition, field_object.id)
// TODO: 他の type の型を使えないか。
pub type FieldObjectPosition = (usize, usize, String);
