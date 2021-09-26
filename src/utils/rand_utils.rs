use rand;

use crate::types::{GetRandom};

pub fn create_get_random() -> GetRandom {
    || { rand::random::<f64>() }
}

pub fn get_ranged_random_integer(get_random: &GetRandom, min: i32, max: i32) -> i32 {
    if min > max {
        panic!("The maximum value must be equal greater than the minimum value.");
    }
    let random_value = get_random();
    let range = (max - min + 1) as f64;
    (random_value * range) as i32 + min
}

#[cfg(test)]
mod tests_of_get_ranged_random_integer {
    use super::*;

    mod when_it_done_not_panic {
        use super::*;

        #[derive(Debug)]
        struct TestCase {
            args: (GetRandom, i32, i32),
            expected: i32,
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: (|| { 0.0 }, 0, 3),
                    expected: 0,
                },
                TestCase {
                    args: (|| { 0.25 }, 0, 3),
                    expected: 1,
                },
                TestCase {
                    args: (|| { 0.5 }, 0, 3),
                    expected: 2,
                },
                TestCase {
                    args: (|| { 0.75 }, 0, 3),
                    expected: 3,
                },
                TestCase {
                    args: (|| { 0.999999 }, 0, 3),
                    expected: 3,
                },
                TestCase {
                    args: (|| { 0.0 }, -2, 2),
                    expected: -2,
                },
                TestCase {
                    args: (|| { 0.20 }, -2, 2),
                    expected: -1,
                },
                TestCase {
                    args: (|| { 0.40 }, -2, 2),
                    expected: 0,
                },
                TestCase {
                    args: (|| { 0.60 }, -2, 2),
                    expected: 1,
                },
                TestCase {
                    args: (|| { 0.80 }, -2, 2),
                    expected: 2,
                },
                TestCase {
                    args: (|| { 0.0 }, 1, 1),
                    expected: 1,
                },
                TestCase {
                    args: (|| { 0.999999 }, 1, 1),
                    expected: 1,
                },
            ];
            for test_case in table {
                assert_eq!(
                    get_ranged_random_integer(&test_case.args.0, test_case.args.1, test_case.args.2),
                    test_case.expected,
                    "Failed in the {:?}.",
                    test_case,
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = " greater than the minimum value")]
    fn it_panics_when_the_max_equals_the_min() {
        let get_random: GetRandom = || { 0.0 };
        get_ranged_random_integer(&get_random, 1, 0);
    }
}

pub fn choice_random_index(get_random: &GetRandom, length: usize) -> usize {
    get_ranged_random_integer(get_random, 0, length as i32 - 1) as usize
}

#[cfg(test)]
mod tests_of_choice_random_index {
    use super::*;

    #[test]
    fn it_works() {
        let get_random_1: GetRandom = || { 0.0 };
        assert_eq!(
            choice_random_index(&get_random_1, 10),
            0,
        );
        let get_random_2: GetRandom = || { 0.999999 };
        assert_eq!(
            choice_random_index(&get_random_2, 10),
            9,
        );
    }
}

/// Shuffle a `Vector` of indexes with the Fisher–Yates algorithm.
pub fn create_shuffled_indexes(get_random: &GetRandom, length: usize) -> Vec<usize> {
    let mut indexes: Vec<usize> = (0..length).collect();
    let mut m = length;
    while m >= 1 {
        let i = (m as f64 * get_random()) as usize;
        m = m - 1;
        let i_value = indexes[i];
        let m_value = indexes[m];
        indexes[m] = i_value;
        indexes[i] = m_value;
    }
    indexes
}

#[cfg(test)]
mod tests_of_create_shuffled_indexes {
    use super::*;

    #[test]
    fn it_works_probably() {
        let get_random = create_get_random();
        for _ in 0..100 {
            let indexes = create_shuffled_indexes(&get_random, 5);
            assert_eq!(indexes.len(), 5);
            assert!(indexes.contains(&0));
            assert!(indexes.contains(&1));
            assert!(indexes.contains(&2));
            assert!(indexes.contains(&3));
            assert!(indexes.contains(&4));
        };
    }
}
