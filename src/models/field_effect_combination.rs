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
    /// It means something like this: `<inner_field_object_id, (outer_field_object_id, Option<FieldElementPosition exists when this is placed>)>`.
    pub id_map: HashMap<u32, (String, Option<FieldElementPosition>)>,
    /// It is the number of frames since it was created. Count from 0.
    pub number_of_frames: u64,
    pub starting_point: FieldElementPosition,
    pub transitions: Vec<TransitionKind>,
}
impl FieldEffectCombination {
    // TODO: 多くの動きが設定できるようにする。
    pub fn new(id_generator: &mut IdGenerator, starting_point: FieldElementPosition, direction: FourDirection) -> Self {
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

        Self {
            id_map: Self::create_id_map_from_transitions(id_generator, &transitions),
            starting_point,
            direction,
            number_of_frames: 0,
            transitions,
        }
    }
    fn create_id_map_from_transitions(
        id_generator: &mut IdGenerator, transitions: &Vec<TransitionKind>
    ) -> HashMap<u32, (String, Option<FieldElementPosition>)> {
        let mut inner_ids: Vec<u32> = vec![];
        for transition in transitions.iter() {
            match transition {
                // TODO: 構造体を含むパターンマッチの時に、ここへ使うフィールドだけ記述したい。
                TransitionKind::Create {inner_field_effect_id, number_of_frames: _, vector: _} => {
                    if !inner_ids.iter().any(|e| e == inner_field_effect_id) {
                        inner_ids.push(inner_field_effect_id.clone());
                    }
                },
                _ => {},
            }
        }
        let mut id_map = HashMap::new();
        for inner_id in inner_ids.iter() {
            id_map.insert(inner_id.clone(), (id_generator.generate_for_field_effect(), None));
        }
        id_map
    }
}
