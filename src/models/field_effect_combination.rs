use std::collections::HashMap;
use std::ops::Range;

use crate::enums::{FourDirection};
use crate::id_generator::IdGenerator;
use crate::types::{FieldElementPosition, XYVector};

// TODO: どこかにまとめる。
const MAX_NUMBER_OF_FRAMES: u64 = std::u64::MAX;

#[derive(Debug)]
pub enum TransitionKind {
    // TODO: FieldEffect の種類を指定できるようにする。
    // TODO: 生成済みの FieldEffect を条件に生成できるようにする。
    Create {
        active_frame: u64,
        inner_field_effect_id: u32,
        /// A vector to determine the location of the destination.
        /// 
        /// Set the vector for the upward pointing case.
        vector: XYVector,
    },
    // TODO: 衝突時の挙動。消滅したりしなかったりする。
    Move {
        active_frames: Range<u64>,
        inner_field_effect_id: u32,
        interval_of_active_frames: u64,
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
        // TODO: 複数の Create へ同じ inner_field_effect_id を設定したらエラーにする。
        // TODO: Move が存在しない inner_field_effect_id を参照していたらエラーにする。
        let transitions: Vec<TransitionKind> = vec![
            TransitionKind::Create {
                inner_field_effect_id: 1,
                active_frame: 1,
                vector: (0, 0),
            },
            TransitionKind::Move {
                inner_field_effect_id: 1,
                active_frames: 3..MAX_NUMBER_OF_FRAMES,
                interval_of_active_frames: 1,
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
                TransitionKind::Create {inner_field_effect_id, active_frame: _, vector: _} => {
                    inner_ids.push(inner_field_effect_id.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_of_create_id_map_from_transitions {
        use super::*;

        #[test]
        fn it_should_return_an_empty_hash_map_when_there_are_no_transitions() {
            let mut id_generator = IdGenerator::new(1);
            let id_map = FieldEffectCombination::create_id_map_from_transitions(&mut id_generator, &vec![]);
            assert!(id_map.is_empty());
        }

        #[test]
        fn it_should_create_an_outer_field_effect_id_from_an_inner_field_effect_id() {
            let mut id_generator = IdGenerator::new(1);
            let id_map = FieldEffectCombination::create_id_map_from_transitions(&mut id_generator, &vec![
                TransitionKind::Create {
                    inner_field_effect_id: 11,
                    active_frame: 1,
                    vector: (0, 0),
                },
            ]);
            assert_eq!(id_map.len(), 1);
            assert_eq!(id_map.get(&11), Some(&(String::from("fe-1"), None)));
        }
        #[test]
        fn it_should_create_ids_from_only_the_create_kind() {
            let mut id_generator = IdGenerator::new(1);
            let id_map = FieldEffectCombination::create_id_map_from_transitions(&mut id_generator, &vec![
                TransitionKind::Create {
                    inner_field_effect_id: 11,
                    active_frame: 1,
                    vector: (0, 0),
                },
                TransitionKind::Create {
                    inner_field_effect_id: 12,
                    active_frame: 1,
                    vector: (0, 0),
                },
                TransitionKind::Move {
                    inner_field_effect_id: 11,
                    active_frames: 0..0,
                    interval_of_active_frames: 1,
                    vector: (0, 0),
                },
            ]);
            assert_eq!(id_map.len(), 2);
            assert_eq!(id_map.get(&11), Some(&(String::from("fe-1"), None)));
            assert_eq!(id_map.get(&12), Some(&(String::from("fe-2"), None)));
        }
    }
}
