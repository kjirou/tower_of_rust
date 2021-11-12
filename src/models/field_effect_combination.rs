#[derive(Debug)]
pub struct FieldEffectCombination {
    /// It is the number of frames since it was created. Count from 0.
    number_of_frames: u64,
}
impl FieldEffectCombination {
    pub fn new() -> Self {
        Self {
            number_of_frames: 0,
        }
    }
}
