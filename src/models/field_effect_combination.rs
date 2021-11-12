use crate::types::{FieldElementPosition, FieldObjectLocation};

// TODO: 少なくとも、FieldObject の生成 -> 移動 -> 衝突で消滅 or 時間で消滅 の遷移が定義できないといけない。


#[derive(Debug)]
pub struct FieldEffectCombination {
    /// It is the number of frames since it was created. Count from 0.
    number_of_frames: u64,
    transitions: Vec<FieldObjectLocation>,
    starting_point: FieldElementPosition,
}
impl FieldEffectCombination {
    pub fn new(starting_point: FieldElementPosition) -> Self {
        Self {
            starting_point,
            number_of_frames: 0,
            transitions: vec![],
        }
    }
}
