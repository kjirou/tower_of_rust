use crate::enums::FourDirection;
use crate::enums::CustomErrorKind;
use crate::types::{FieldElementPosition, RectangleSize, XYCoordinates, XYLocation, XYVector};

pub mod dungeon_generator;
pub mod rand_utils;

// TODO: Err(error) でエラー種別の判定をしたくて作った。一般的に Rust でどうするのか不明。
#[derive(Debug)]
pub struct CustomError {
    pub kind: CustomErrorKind,
}

pub fn create_four_directions() -> Vec<FourDirection> {
    vec![
        FourDirection::Up,
        FourDirection::Right,
        FourDirection::Down,
        FourDirection::Left,
    ]
}

pub fn translate_coordinate(start: &XYCoordinates, vector: &XYVector) -> XYCoordinates {
    (start.0 + vector.0, start.1 + vector.1)
}

#[cfg(test)]
mod tests_of_translate_coordinate {
    use super::*;

    struct TestCase {
        args: (XYCoordinates, XYVector),
        expected: XYCoordinates,
    }

    #[test]
    fn it_works() {
        let table: Vec::<TestCase> = vec![
            TestCase {
                args: ((1, 2), (10, 20)),
                expected: (11, 22),
            },
            TestCase {
                args: ((1, 2), (-10, -20)),
                expected: (-9, -18),
            },
            TestCase {
                args: ((1, 2), (0, 0)),
                expected: (1, 2),
            },
        ];
        for test_case in table {
            assert_eq!(
                translate_coordinate(&test_case.args.0, &test_case.args.1),
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
    field_size: &RectangleSize, start: &FieldElementPosition, direction: &FourDirection
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
    Ok((moved.0 as u32, moved.1 as u32))
}

#[cfg(test)]
mod tests_of_translate_position_by_direction {
    use super::*;

    mod when_it_returns_ok {
        use super::*;

        struct TestCase {
            args: (RectangleSize, FieldElementPosition, FourDirection),
            expected: FieldElementPosition,
        }

        #[test]
        fn it_works() {
            let table: Vec::<TestCase> = vec![
                TestCase {
                    args: ((99, 99), (1, 2), FourDirection::Up),
                    expected: (1, 1),
                },
                TestCase {
                    args: ((99, 99), (1, 2), FourDirection::Right),
                    expected: (2, 2),
                },
                TestCase {
                    args: ((99, 99), (1, 2), FourDirection::Down),
                    expected: (1, 3),
                },
                TestCase {
                    args: ((99, 99), (1, 2), FourDirection::Left),
                    expected: (0, 2),
                },
            ];
            for test_case in table {
                assert_eq!(
                    translate_position_by_direction(&test_case.args.0, &test_case.args.1, &test_case.args.2).unwrap(),
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
                    translate_position_by_direction(test_case.args.0, test_case.args.1, &test_case.args.2).unwrap_err().kind,
                    CustomErrorKind::CoordinateIsOutsideOfPosition,
                );
            }
        }
    }
}

pub fn compute_map_xy_on_field(map_size: &RectangleSize, map_center: &FieldElementPosition) -> XYCoordinates {
    if map_size.0 % 2 == 0 || map_size.1 % 2 == 0 {
        panic!("The size of map rectangle is odd only.");
    }
    let offset = (map_size.0 / 2, map_size.1 / 2);
    (
        (map_center.0 as i32 - offset.0 as i32),
        (map_center.1 as i32 - offset.1 as i32),
    )
}

#[cfg(test)]
mod tests_of_compute_map_xy_on_field {
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
                    compute_map_xy_on_field(test_case.args.0, test_case.args.1),
                    test_case.expected,
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = " is odd only")]
    fn it_panics_when_the_width_of_the_rectangle_is_even() {
        compute_map_xy_on_field(&(2, 1), &(0, 0));
    }

    #[test]
    #[should_panic(expected = " is odd only")]
    fn it_panics_when_the_height_of_the_rectangle_is_even() {
        compute_map_xy_on_field(&(1, 2), &(0, 0));
    }
}

pub fn add_paddings_to_location(location: &XYLocation, paddings: u32) -> XYLocation {
    (
        (location.0.0 - paddings as i32, location.0.1 - paddings as i32),
        (location.1.0 + paddings * 2, location.1.1 + paddings * 2),
    )
}

#[cfg(test)]
mod tests_of_add_paddings_to_location {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        args: (XYLocation, u32),
        expected: XYLocation,
    }

    #[test]
    fn it_works() {
        let table = [
            //  012345
            // 0 E---+
            // 1 |A-+|
            // 2 |+-+|
            // 3 +---+
            TestCase {
                args: (((2, 1), (3, 2)), 1),
                expected: ((1, 0), (5, 4)),
            },
            // -543210
            // 3 E---+
            // 2 |A-+|
            // 1 |+-+|
            // 0 +---+
            TestCase {
                args: (((-3, -2), (3, 2)), 1),
                expected: ((-4, -3), (5, 4)),
            },
            //  01234
            // 0E---+
            // 1|   |
            // 2| A |
            // 3|   |
            // 4+---+
            TestCase {
                args: (((2, 2), (1, 1)), 2),
                expected: ((0, 0), (5, 5)),
            },
            // No paddings.
            TestCase {
                args: (((2, 1), (3, 2)), 0),
                expected: ((2, 1), (3, 2)),
            },
        ];
        for test_case in table {
            assert_eq!(
                add_paddings_to_location(&test_case.args.0, test_case.args.1),
                test_case.expected,
                "Failed in the {:?}.",
                test_case,
            );
        }
    }
}

pub fn do_rectangles_overlap(
    a_location: &XYLocation,
    b_location: &XYLocation,
) -> bool {
    let a_bottom_right: XYCoordinates = (a_location.0.0 + a_location.1.0 as i32 - 1, a_location.0.1 + a_location.1.1 as i32 - 1);
    let b_bottom_right: XYCoordinates = (b_location.0.0 + b_location.1.0 as i32 - 1, b_location.0.1 + b_location.1.1 as i32 - 1);
    let a_x_range = a_location.0.0..=a_bottom_right.0;
    let a_y_range = a_location.0.1..=a_bottom_right.1;
    let b_x_range = b_location.0.0..=b_bottom_right.0;
    let b_y_range = b_location.0.1..=b_bottom_right.1;
    (
        a_x_range.contains(&b_location.0.0) ||
        a_x_range.contains(&b_bottom_right.0) ||
        b_x_range.contains(&a_location.0.0) ||
        b_x_range.contains(&a_bottom_right.0)
    ) && (
        a_y_range.contains(&b_location.0.1) ||
        a_y_range.contains(&b_bottom_right.1) ||
        b_y_range.contains(&a_location.0.1) ||
        b_y_range.contains(&a_bottom_right.1)
    )
}

#[cfg(test)]
mod tests_of_do_rectangles_overlap {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        args: (XYLocation, XYLocation),
        expected: bool,
    }

    #[test]
    fn it_works() {
        let table = [
            //  01234
            // 0
            // 1  B-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((2, 1), (1, 1))),
                expected: true,
            },
            //  01234
            // 0
            // 1  A-B
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((4, 1), (1, 1))),
                expected: true,
            },
            //  01234
            // 0
            // 1  A-+
            // 2  B-+
            TestCase {
                args: (((2, 1), (3, 2)), ((2, 2), (1, 1))),
                expected: true,
            },
            //  01234
            // 0
            // 1  A-+
            // 2  +-B
            TestCase {
                args: (((2, 1), (3, 2)), ((4, 2), (1, 1))),
                expected: true,
            },
            //  01234
            // 0
            // 1 BA-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((1, 1), (1, 1))),
                expected: false,
            },
            //  01234
            // 0  B
            // 1  A-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((2, 0), (1, 1))),
                expected: false,
            },
            //  012345
            // 0
            // 1  A-+B
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((5, 1), (1, 1))),
                expected: false,
            },
            //  01234
            // 0    B
            // 1  A-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((4, 0), (1, 1))),
                expected: false,
            },
            //  01234
            // 0
            // 1  A-+
            // 2 B+-+
            TestCase {
                args: (((2, 1), (3, 2)), ((1, 2), (1, 1))),
                expected: false,
            },
            //  01234
            // 0
            // 1  A-+
            // 2  +-+
            // 3  B
            TestCase {
                args: (((2, 1), (3, 2)), ((2, 3), (1, 1))),
                expected: false,
            },
            //  012345
            // 0
            // 1  A-+
            // 2  +-+B
            TestCase {
                args: (((2, 1), (3, 2)), ((5, 2), (1, 1))),
                expected: false,
            },
            //  01234
            // 0
            // 1  A-+
            // 2  +-+
            // 3    B
            TestCase {
                args: (((2, 1), (3, 2)), ((4, 3), (1, 1))),
                expected: false,
            },
            //  01234
            // 0B-+
            // 1+-A-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((0, 0), (3, 2))),
                expected: true,
            },
            //  01234
            // 0A-+
            // 1+-B-+
            // 2  +-+
            TestCase {
                args: (((0, 0), (3, 2)), ((2, 1), (3, 2))),
                expected: true,
            },
            //  012345
            // 0B-+
            // 1+-+A-+
            // 2   +-+
            TestCase {
                args: (((3, 1), (3, 2)), ((0, 0), (3, 2))),
                expected: false,
            },
            //  012345
            // 0A-+
            // 1+-+B-+
            // 2   +-+
            TestCase {
                args: (((0, 0), (3, 2)), ((3, 1), (3, 2))),
                expected: false,
            },
            //  012345
            // 0 B---+
            // 1 |A-+|
            // 2 |+-+|
            // 3 +---+
            TestCase {
                args: (((2, 1), (3, 2)), ((1, 0), (5, 4))),
                expected: true,
            },
            //  012345
            // 0 A---+
            // 1 |B-+|
            // 2 |+-+|
            // 3 +---+
            TestCase {
                args: (((1, 0), (5, 4)), ((2, 1), (3, 2))),
                expected: true,
            },
            //  01234
            // 0
            // 1  +-+
            // 2  +-+
            TestCase {
                args: (((2, 1), (3, 2)), ((2, 1), (3, 2))),
                expected: true,
            },
            // -43210
            // 3B-+
            // 2+-A-+
            // 1  +-+
            TestCase {
                args: (((-2, -2), (3, 2)), ((-4, -3), (3, 2))),
                expected: true,
            },
            // -43210
            // 3A-+
            // 2+-B-+
            // 1  +-+
            TestCase {
                args: (((-4, -3), (3, 2)), ((-2, -2), (3, 2))),
                expected: true,
            },
            // -543210
            // 2B-+
            // 1+-+A-+
            // 0   +-+
            TestCase {
                args: (((-2, -1), (3, 2)), ((-5, -2), (3, 2))),
                expected: false,
            },
            // -543210
            // 2A-+
            // 1+-+B-+
            // 0   +-+
            TestCase {
                args: (((-5, -2), (3, 2)), ((-2, -1), (3, 2))),
                expected: false,
            },
        ];
        for test_case in table {
            assert_eq!(
                do_rectangles_overlap(&test_case.args.0, &test_case.args.1),
                test_case.expected,
                "Failed in the {:?}.",
                test_case,
            );
        }
    }
}
