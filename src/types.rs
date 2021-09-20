/// The singleton function to generate random numbers.
/// 
/// Only one function is generated and shared throughout the application.  
/// It seems that [ThreadRng](https://docs.rs/rand/0.8.4/rand/rngs/struct.ThreadRng.html) is the correct target for sharing, but I didn't know how to handle it well.
pub type GetRandom = fn() -> f64;

/// (width, height)
pub type RectangleSize = (u32, u32);

/// (x, y)
pub type XYCoordinates = (i32, i32);

/// (x, y)
pub type XYVector = (i32, i32);

/// (x, y) coordinates of a `FieldElement`.
pub type FieldElementPosition = (u32, u32);

/// A location of a `FieldObject`.
/// 
/// It consists of (x, y, id_of_field_object).  
/// The (x, y) part is equal to a `FieldElementPosition`.
// TODO: 他の type の型を使えないか。
pub type FieldObjectLocation = (u32, u32, String);
