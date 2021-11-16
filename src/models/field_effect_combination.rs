use std::collections::HashMap;

use crate::enums::{FourDirection};
use crate::id_generator::IdGenerator;
use crate::types::{FieldElementPosition, FieldEffectLocation, XYVector};


// TODO: 少なくとも、FieldObject の生成 -> 移動 -> 衝突で消滅 or 時間で消滅 の遷移が定義できないといけない。

#[derive(Debug)]
pub enum TransitionKind {
    // TODO: FieldEffect の種類を指定できるようにする。
    // TODO: 生成済みの FieldEffect を条件に生成できるようにする。
    Create {
        inner_field_effect_id: u32,
        number_of_frames: u64,
        /// A vector to determine the location of the destination.
        /// 
        /// Set the vector for the upward pointing case.
        vector: XYVector,
    },
    // TODO: 衝突時の挙動。消滅したりしなかったりする。
    Move {
        inner_field_effect_id: u32,
        number_of_frames: u64,
        /// A vector indicating the destination.
        /// 
        /// Set the vector for the upward pointing case.
        vector: XYVector,
    },
}

#[derive(Debug)]
pub struct FieldEffectCombination {
    pub direction: FourDirection,
    pub id_map: HashMap<u32, FieldEffectLocation>,
    /// It is the number of frames since it was created. Count from 0.
    pub number_of_frames: u64,
    pub starting_point: FieldElementPosition,
    pub transitions: Vec<TransitionKind>,
}
impl FieldEffectCombination {
    // TODO: 多くの動きが設定できるようにする。
    pub fn new(id_generator: IdGenerator, starting_point: FieldElementPosition, direction: FourDirection) -> Self {
        let transitions: Vec<TransitionKind> = vec![
            TransitionKind::Create {
                inner_field_effect_id: 1,
                number_of_frames: 1,
                vector: (0, 0),
            },
            TransitionKind::Move {
                inner_field_effect_id: 1,
                number_of_frames: 3,
                vector: (0, -1),
            },
            TransitionKind::Move {
                inner_field_effect_id: 1,
                number_of_frames: 5,
                vector: (0, -1),
            },
        ];

        // TODO: Create を拾って、外向けの FieldEffect 用の id を生成する。

        let mut id_map: HashMap<u32, FieldEffectLocation> = HashMap::new();
        Self {
            id_map: id_map,
            starting_point,
            direction,
            number_of_frames: 0,
            transitions,
        }
    }
    fn prepare_field_effect_ids() -> Vec<String> {
        vec![]
    }
}
