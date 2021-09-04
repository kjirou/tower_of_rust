use crate::enums::FourDirection;
use crate::enums::CustomErrorKind;
use crate::types::{FieldElementPosition, FieldObjectPosition};

// TODO: Err(error) でエラー種別の判定をしたくて作った。一般的に Rust でどうするのか不明。
#[derive(Debug)]
pub struct CustomError {
    pub kind: CustomErrorKind,
}

pub fn xyi_to_xy(xyi: &FieldObjectPosition) -> FieldElementPosition {
    (xyi.0, xyi.1)
}

#[cfg(test)]
mod tests_of_xyi_to_xy {
    use super::*;

    struct TestCase {
        args: (FieldObjectPosition,),
        expected: FieldElementPosition,
    }

    #[test]
    fn it_works() {
        let table: Vec::<TestCase> = vec![
            TestCase {
                args: ((1, 2, String::from("foo")),),
                expected: (1, 2),
            },
        ];
        for test_case in table {
            assert_eq!(
                xyi_to_xy(&test_case.args.0),
                test_case.expected,
                "{:?} => {:?}",
                test_case.args.0,
                test_case.expected,
            );
        }
    }
}

pub fn translate_coordinate(start: &(i32, i32), vector: &(i32, i32)) -> (i32, i32) {
    (start.0 + vector.0, start.1 + vector.1)
}

#[cfg(test)]
mod tests_of_translate_coordinate {
    use super::*;

    struct TestCase<'a> {
        args: (&'a (i32, i32), &'a (i32, i32)),
        expected: (i32, i32),
    }

    #[test]
    fn it_works() {
        let table: Vec::<TestCase> = vec![
            TestCase {
                args: (&(1, 2), &(10, 20)),
                expected: (11, 22),
            },
            TestCase {
                args: (&(1, 2), &(-10, -20)),
                expected: (-9, -18),
            },
            TestCase {
                args: (&(1, 2), &(0, 0)),
                expected: (1, 2),
            },
        ];
        for test_case in table {
            assert_eq!(
                translate_coordinate(test_case.args.0, test_case.args.1),
                test_case.expected,
                "{:?} + {:?} => {:?}",
                test_case.args.0,
                test_case.args.1,
                test_case.expected,
            );
        }
    }
}

// TODO: 最大のxyを指定できるようにする。
pub fn translate_position_by_direction(
    start: &FieldElementPosition, direction: FourDirection
) -> Result<FieldElementPosition, CustomError> {
    let vector: (i32, i32) = match direction {
        FourDirection::Up => (0, -1),
        FourDirection::Right => (1, 0),
        FourDirection::Down => (0, 1),
        FourDirection::Left => (-1, 0),
    };
    let moved = translate_coordinate(&(start.0 as i32, start.1 as i32), &vector);
    if moved.0 < 0 || moved.1 < 0 {
        return Err(CustomError {
            kind: CustomErrorKind::CoordinateIsOutsideOfPosition,
        });
    }
    Ok((moved.0 as usize, moved.1 as usize))
}

#[cfg(test)]
mod tests_of_translate_position_by_direction {
    use super::*;

    mod when_it_returns_ok {
        use super::*;

        struct TestCase<'a> {
            args: (&'a (usize, usize), FourDirection),
            expected: (usize, usize),
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: (&(1, 2), FourDirection::Up),
                    expected: (1, 1),
                },
                TestCase {
                    args: (&(1, 2), FourDirection::Right),
                    expected: (2, 2),
                },
                TestCase {
                    args: (&(1, 2), FourDirection::Down),
                    expected: (1, 3),
                },
                TestCase {
                    args: (&(1, 2), FourDirection::Left),
                    expected: (0, 2),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_position_by_direction(test_case.args.0, test_case.args.1).unwrap(),
                    test_case.expected,
                );
            }
        }
    }

    mod when_it_returns_custom_error {
        use super::*;

        struct TestCase<'a> {
            args: (&'a (usize, usize), FourDirection),
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: (&(1, 0), FourDirection::Up),
                },
                TestCase {
                    args: (&(0, 1), FourDirection::Left),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_position_by_direction(test_case.args.0, test_case.args.1).unwrap_err().kind,
                    CustomErrorKind::CoordinateIsOutsideOfPosition,
                );
            }
        }
    }
}
