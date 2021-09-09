use crate::enums::FourDirection;
use crate::enums::CustomErrorKind;
use crate::types::{FieldElementPosition, FieldObjectLocation, RectangleSize, XYCoordinates, XYVector};

// TODO: Err(error) でエラー種別の判定をしたくて作った。一般的に Rust でどうするのか不明。
#[derive(Debug)]
pub struct CustomError {
    pub kind: CustomErrorKind,
}

// TODO: fol_to_fep とかの方がまだ良さそう。
pub fn xyi_to_xy(xyi: &FieldObjectLocation) -> FieldElementPosition {
    (xyi.0, xyi.1)
}

#[cfg(test)]
mod tests_of_xyi_to_xy {
    use super::*;

    struct TestCase {
        args: (FieldObjectLocation,),
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

pub fn translate_coordinate(start: &XYCoordinates, vector: &XYVector) -> XYCoordinates {
    (start.0 + vector.0, start.1 + vector.1)
}

#[cfg(test)]
mod tests_of_translate_coordinate {
    use super::*;

    struct TestCase<'a> {
        args: (&'a XYCoordinates, &'a XYVector),
        expected: XYCoordinates,
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

pub fn translate_position_by_direction(
    field_size: &RectangleSize, start: &FieldElementPosition, direction: FourDirection
) -> Result<FieldElementPosition, CustomError> {
    let vector: XYVector = match direction {
        FourDirection::Up => (0, -1),
        FourDirection::Right => (1, 0),
        FourDirection::Down => (0, 1),
        FourDirection::Left => (-1, 0),
    };
    let moved = translate_coordinate(&(start.0 as i32, start.1 as i32), &vector);
    if moved.0 < 0 ||
        moved.1 < 0 ||
        moved.0 >= field_size.0 as i32 ||
        moved.1 >= field_size.1 as i32 {
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
            args: (&'a RectangleSize, &'a FieldElementPosition, FourDirection),
            expected: (usize, usize),
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: (&(99, 99), &(1, 2), FourDirection::Up),
                    expected: (1, 1),
                },
                TestCase {
                    args: (&(99, 99), &(1, 2), FourDirection::Right),
                    expected: (2, 2),
                },
                TestCase {
                    args: (&(99, 99), &(1, 2), FourDirection::Down),
                    expected: (1, 3),
                },
                TestCase {
                    args: (&(99, 99), &(1, 2), FourDirection::Left),
                    expected: (0, 2),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_position_by_direction(test_case.args.0, test_case.args.1, test_case.args.2).unwrap(),
                    test_case.expected,
                );
            }
        }
    }

    mod when_it_returns_custom_error {
        use super::*;

        struct TestCase<'a> {
            args: (&'a RectangleSize, &'a FieldElementPosition, FourDirection),
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: (&(99, 99), &(1, 0), FourDirection::Up),
                },
                TestCase {
                    args: (&(99, 99), &(0, 1), FourDirection::Left),
                },
                TestCase {
                    args: (&(20, 10), &(20, 0), FourDirection::Right),
                },
                TestCase {
                    args: (&(20, 10), &(0, 10), FourDirection::Down),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_position_by_direction(test_case.args.0, test_case.args.1, test_case.args.2).unwrap_err().kind,
                    CustomErrorKind::CoordinateIsOutsideOfPosition,
                );
            }
        }
    }
}

pub fn translate_rectangle_on_field(size: &RectangleSize, target_of_interest: &FieldElementPosition) -> XYCoordinates {
    if size.0 % 2 == 0 || size.1 % 2 == 0 {
        panic!("The size of the rectangle is odd only.");
    }
    let offset = (size.0 / 2, size.1 / 2);
    (
        (target_of_interest.0 as i32 - offset.0 as i32),
        (target_of_interest.1 as i32 - offset.1 as i32),
    )
}

#[cfg(test)]
mod tests_of_translate_rectangle_on_field {
    use super::*;

    mod when_it_does_not_panic {
        use super::*;

        struct TestCase<'a> {
            args: (&'a RectangleSize, &'a FieldElementPosition),
            expected: (i32, i32),
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                // X---+
                // | @-|- (0 ,0) at the field
                // +---+
                //   |
                TestCase {
                    args: (&(5, 3), &(0, 0)),
                    expected: (-2, -1),
                },
                // +-------
                // | X---+
                // | | @ |  (4 ,2) at the field
                // | +---+
                // |
                TestCase {
                    args: (&(5, 3), &(4, 2)),
                    expected: (2, 1),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_rectangle_on_field(test_case.args.0, test_case.args.1),
                    test_case.expected,
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = " is odd only")]
    fn it_panics_when_the_width_of_the_rectangle_is_even() {
        translate_rectangle_on_field(&(2, 1), &(0, 0));
    }

    #[test]
    #[should_panic(expected = " is odd only")]
    fn it_panics_when_the_height_of_the_rectangle_is_even() {
        translate_rectangle_on_field(&(1, 2), &(0, 0));
    }
}
