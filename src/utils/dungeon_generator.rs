use termion;

use crate::enums::FourDirection;
use crate::types::{GetRandom, RectangleSize, XYCoordinates, XYLocation};
use crate::utils::{add_paddings_to_location, create_four_directions, do_rectangles_overlap, rand_utils};

/// (x, y) coordinates in the dungeon. The top-left position is (0, 0).
type DungeonCellPosition = (u32, u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DungeonSpaceKind {
    Entrance,
    Passage,
    Room,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DungeonCellKind {
    Blank,
    Entrance,
    Passage,
    Room,
    Wall,
}

fn relocate_xy_to_position(dungeon_size: &RectangleSize, xy: &XYCoordinates) -> Option<DungeonCellPosition> {
    let x_range = 0..(dungeon_size.0 as i32);
    let y_range = 0..(dungeon_size.1 as i32);
    if x_range.contains(&xy.0) && y_range.contains(&xy.1) {
        Some((xy.0 as u32, xy.1 as u32))
    } else {
        None
    }
}

#[cfg(test)]
mod tests_of_relocate_xy_to_position {
    use super::*;

    mod when_it_returns_some_value {
        use super::*;

        struct TestCase {
            args: (RectangleSize, XYCoordinates),
            expected: DungeonCellPosition,
        }

        #[test]
        fn it_works() {
            let table = vec![
                TestCase {
                    args: ((3, 2), (0, 0)),
                    expected: (0, 0),
                },
                TestCase {
                    args: ((3, 2), (2, 1)),
                    expected: (2, 1),
                },
            ];
            for test_case in table {
                assert_eq!(relocate_xy_to_position(&test_case.args.0, &test_case.args.1).unwrap(), test_case.expected);
            }
        }
    }

    mod when_it_returns_none {
        use super::*;

        struct TestCase {
            args: (RectangleSize, XYCoordinates),
        }

        #[test]
        fn it_works() {
            let table = vec![
                TestCase {
                    args: ((3, 2), (-1, 0)),
                },
                TestCase {
                    args: ((3, 2), (0, -1)),
                },
                TestCase {
                    args: ((3, 2), (3, 1)),
                },
                TestCase {
                    args: ((3, 2), (2, 2)),
                },
            ];
            for test_case in table {
                assert_eq!(relocate_xy_to_position(&test_case.args.0, &test_case.args.1).is_none(), true);
            }
        }
    }
}

fn random_space_size(
    get_random: &GetRandom,
    min_size: &RectangleSize,
    max_size: &RectangleSize,
) -> RectangleSize {
    (
        rand_utils::get_ranged_random_integer(get_random, min_size.0 as i32, max_size.0 as i32) as u32,
        rand_utils::get_ranged_random_integer(get_random, min_size.1 as i32, max_size.1 as i32) as u32,
    )
}

#[cfg(test)]
mod tests_of_random_space_size {
    use super::*;

    #[test]
    fn it_works() {
        let get_random_1: GetRandom = || { 0.0 };
        assert_eq!(
            random_space_size(&get_random_1, &(2, 3), &(4, 5)),
            (2, 3),
        );

        let get_random_2: GetRandom = || { 0.999999 };
        assert_eq!(
            random_space_size(&get_random_2, &(2, 3), &(4, 5)),
            (4, 5),
        );
    }
}

fn create_candidates_of_inner_rectangle_position(base_size: &RectangleSize, inner_size: &RectangleSize) -> Vec<DungeonCellPosition> {
    let mut positions: Vec<DungeonCellPosition> = vec![];
    if base_size.0 < inner_size.0 || base_size.1 < inner_size.1 {
        return positions;
    }
    let width_delta = base_size.0 - inner_size.0 + 1;
    let height_delta = base_size.1 - inner_size.1 + 1;
    for y in 0..height_delta {
        for x in 0..width_delta {
            positions.push((x, y));
        }
    }
    positions
}

#[cfg(test)]
mod tests_of_create_candidates_of_inner_rectangle_position {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        args: (RectangleSize, RectangleSize),
        expected: Vec<DungeonCellPosition>,
    }

    #[test]
    fn it_works() {
        let table = [
            TestCase {
                args: ((3, 2), (3, 2)),
                expected: vec![(0, 0)],
            },
            TestCase {
                args: ((2, 2), (3, 2)),
                expected: vec![],
            },
            TestCase {
                args: ((3, 1), (3, 2)),
                expected: vec![],
            },
            TestCase {
                args: ((5, 2), (3, 2)),
                expected: vec![(0, 0), (1, 0), (2, 0)],
            },
            TestCase {
                args: ((3, 4), (3, 2)),
                expected: vec![(0, 0), (0, 1), (0, 2)],
            },
            TestCase {
                args: ((5, 3), (3, 2)),
                expected: vec![
                    (0, 0), (1, 0), (2, 0),
                    (0, 1), (1, 1), (2, 1),
                ],
            },
        ];
        for test_case in table {
            assert_eq!(
                create_candidates_of_inner_rectangle_position(&test_case.args.0, &test_case.args.1),
                test_case.expected,
                "Failed in the {:?}.",
                test_case,
            );
        }
    }
}

#[derive(Debug)]
pub struct DungeonSpace {
    /// The value of how many times it was generated from the starting space. Count from `0`.
    pub depth: u32,
    pub kind: DungeonSpaceKind,
    /// The top-left position in space. Does not include walls. It is unique among all spaces.
    pub position: DungeonCellPosition,
    pub size: RectangleSize,
}
impl DungeonSpace {
    fn get_position_as_xy(&self) -> XYCoordinates {
        (self.position.0 as i32, self.position.1 as i32)
    }
    fn get_location_as_xy(&self, wall_length: u32) -> XYLocation {
        add_paddings_to_location(&(self.get_position_as_xy(), self.size), wall_length)
    }
    fn get_bottom_right_position(&self) -> DungeonCellPosition {
        (self.position.0 + self.size.0 - 1, self.position.1 + self.size.1 - 1)
    }
    fn is_overlapping_to_others(&self, wall_length: u32, other_locations: &Vec<XYLocation>) -> bool {
        let location = self.get_location_as_xy(wall_length);
        other_locations.iter().any(|other| {
            do_rectangles_overlap(&location, other)
        })
    }
    fn create_candidates_of_next_entrance_position(&self, dungeon_size: &RectangleSize, direction: &FourDirection) -> Vec<DungeonCellPosition> {
        let xy_list: Vec<XYCoordinates> = match direction {
            &FourDirection::Up => {
                (0..self.size.0 as i32)
                    .map(|x| (
                        self.position.0 as i32 + x,
                        self.position.1 as i32 - 1,
                    ))
                    .collect()
            },
            &FourDirection::Right => {
                (0..self.size.1 as i32)
                    .map(|y| (
                        self.get_bottom_right_position().0 as i32 + 1,
                        self.position.1 as i32 + y,
                    ))
                    .collect()
            },
            &FourDirection::Down => {
                (0..self.size.0 as i32)
                    .map(|x| (
                        self.position.0 as i32 + x,
                        self.get_bottom_right_position().1 as i32 + 1,
                    ))
                    .collect()
            },
            &FourDirection::Left => {
                (0..self.size.1 as i32)
                    .map(|y| (
                        self.position.0 as i32 - 1,
                        self.position.1 as i32 + y,
                    ))
                    .collect()
            },
        };
        xy_list.iter().filter_map(|e| relocate_xy_to_position(dungeon_size, e)).collect()
    }
}

#[cfg(test)]
mod tests_of_dungeon_space {
    use super::*;

    mod tests_of_is_overlapping_to_others {
        use super::*;

        #[derive(Debug)]
        struct TestCase {
            args: (u32, Vec<XYLocation>),
            expected: bool,
            instance: DungeonSpace,
            name: String,
        }

        fn create_test_instance() -> DungeonSpace {
            DungeonSpace {
                position: (0 ,0),
                size: (1, 1),
                kind: DungeonSpaceKind::Room,
                depth: 0,
            }
        }

        #[test]
        fn it_works() {
            let table = [
                TestCase {
                    name: String::from("it returns false when it does not overlap anywhere"),
                    instance: DungeonSpace {
                        position: (0, 0),
                        size: (1, 1),
                        ..create_test_instance()
                    },
                    args: (
                        0,
                        vec![
                            ((-1, 0), (1, 1)),
                            ((1, 0), (1, 1)),
                            ((2, 0), (1, 1)),
                        ]
                    ),
                    expected: false,
                },
                TestCase {
                    name: String::from("it returns true when it overlap the first"),
                    instance: DungeonSpace {
                        position: (0, 0),
                        size: (1, 1),
                        ..create_test_instance()
                    },
                    args: (
                        0,
                        vec![
                            ((0, 0), (1, 1)),
                            ((1, 0), (1, 1)),
                            ((2, 0), (1, 1)),
                        ]
                    ),
                    expected: true,
                },
                TestCase {
                    name: String::from("it returns true when it overlap the last"),
                    instance: DungeonSpace {
                        position: (0, 0),
                        size: (1, 1),
                        ..create_test_instance()
                    },
                    args: (
                        0,
                        vec![
                            ((-2, 0), (1, 1)),
                            ((-1, 0), (1, 1)),
                            ((0, 0), (1, 1)),
                        ]
                    ),
                    expected: true,
                },
            ];
            for test_case in table {
                assert_eq!(
                    test_case.instance.is_overlapping_to_others(test_case.args.0, &test_case.args.1),
                    test_case.expected,
                    "Failed in the \"{}\".",
                    test_case.name,
                );
            }
        }
    }

    mod tests_of_create_candidates_of_next_entrance_position {
        use super::*;

        #[derive(Debug)]
        struct TestCase {
            args: (RectangleSize, FourDirection),
            instance: DungeonSpace,
            expected: Vec<DungeonCellPosition>,
        }

        fn create_test_instance() -> DungeonSpace {
            DungeonSpace {
                position: (2, 1),
                kind: DungeonSpaceKind::Room,
                size: (3, 2),
                depth: 0,
            }
        }

        #[test]
        fn it_works() {
            let table = vec![
                //  01234
                // 0  EEE
                // 1  P-+
                // 2  +-+
                TestCase {
                    instance: create_test_instance(),
                    args: ((99, 99), FourDirection::Up),
                    expected: vec![(2, 0), (3, 0), (4, 0)],
                },
                //  012345
                // 0
                // 1  P-+E
                // 2  +-+E
                TestCase {
                    instance: create_test_instance(),
                    args: ((99, 99), FourDirection::Right),
                    expected: vec![(5, 1), (5, 2)],
                },
                //  01234
                // 0
                // 1  P-+
                // 2  +-+
                // 3  EEE
                TestCase {
                    instance: create_test_instance(),
                    args: ((99, 99), FourDirection::Down),
                    expected: vec![(2, 3), (3, 3), (4, 3)],
                },
                //  01234
                // 0
                // 1 EP-+
                // 2 E+-+
                TestCase {
                    instance: create_test_instance(),
                    args: ((99, 99), FourDirection::Left),
                    expected: vec![(1, 1), (1, 2)],
                },
                //  01234
                // 0  P-+
                // 1  +-+
                TestCase {
                    instance: DungeonSpace {
                        position: (2, 0),
                        ..create_test_instance()
                    },
                    args: ((99, 99), FourDirection::Up),
                    expected: vec![],
                },
                //  012345
                // 0     X
                // 1  P-+X
                // 2  +-+X
                TestCase {
                    instance: create_test_instance(),
                    args: ((4, 99), FourDirection::Right),
                    expected: vec![],
                },
                //  01234
                // 0
                // 1  P-+
                // 2  +-+
                // 3XXXXX
                TestCase {
                    instance: create_test_instance(),
                    args: ((99, 3), FourDirection::Down),
                    expected: vec![],
                },
                //  012
                // 0 
                // 1P-+
                // 2+-+
                TestCase {
                    instance: DungeonSpace {
                        position: (0, 1),
                        ..create_test_instance()
                    },
                    args: ((99, 99), FourDirection::Left),
                    expected: vec![],
                },
            ];
            for test_case in table {
                assert_eq!(
                    test_case.instance.create_candidates_of_next_entrance_position(&test_case.args.0, &test_case.args.1),
                    test_case.expected,
                    "Failed in the {:?}.",
                    test_case,
                );
            };
        }
    }
}

#[derive(Debug)]
pub struct DungeonCell {
    pub kind: DungeonCellKind,
    pub position: DungeonCellPosition,
    pub related_space_position: Option<DungeonCellPosition>,
}

fn create_matrix(size: &RectangleSize) -> Vec<Vec<DungeonCell>> {
    let mut matrix: Vec<Vec<DungeonCell>> = vec![];
    for y in 0..size.1 {
        let mut row: Vec<DungeonCell> = vec![];
        for x in 0..size.0 {
            row.push(DungeonCell {
                position: (x, y),
                kind: DungeonCellKind::Blank,
                related_space_position: None,
            });
        }
        matrix.push(row);
    }
    matrix
}

pub struct Dungeon {
    pub matrix: Vec<Vec<DungeonCell>>,
    pub spaces: Vec<DungeonSpace>,
}
impl Dungeon {
    pub fn new(dungeon_size: &RectangleSize, spaces: &Vec<DungeonSpace>, margin_length: u32) -> Self {
        // Combine margins.
        let new_dungeon_size = (dungeon_size.0 + margin_length * 2, dungeon_size.1 + margin_length * 2);
        let new_spaces: Vec<DungeonSpace> = spaces.iter()
            .map(|space| {
                DungeonSpace {
                    position: (space.position.0 + margin_length, space.position.1 + margin_length),
                    size: space.size,
                    kind: space.kind,
                    depth: space.depth,
                }
            })
            .collect();
    
        // Create a matrix of cells.
        let mut matrix = create_matrix(&new_dungeon_size);
    
        // Relate spaces to cells.
        for space in &new_spaces {
            for y in space.position.1..(space.position.1 + space.size.1) {
                for x in space.position.0..(space.position.0 + space.size.0) {
                    let y = y as usize;
                    let x = x as usize;
                    matrix[y][x].kind = match space.kind {
                        DungeonSpaceKind::Entrance => DungeonCellKind::Entrance,
                        DungeonSpaceKind::Passage => DungeonCellKind::Passage,
                        DungeonSpaceKind::Room => DungeonCellKind::Room,
                    };
                    matrix[y][x].related_space_position = Some(space.position);
                }
            }
        }
    
        // Set walls around each space.
        let direction_vectors = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
        for y in 0..new_dungeon_size.1 {
            for x in 0..new_dungeon_size.0 {
                let xu = x as usize;
                let yu = y as usize;
                if matrix[yu][xu].kind != DungeonCellKind::Blank {
                    continue;
                }
                for vector in direction_vectors {
                    let is_wall = match relocate_xy_to_position(&new_dungeon_size, &(x as i32 + vector.0, y as i32 + vector.1)) {
                        Some(adjacent_position) => {
                            match &matrix[adjacent_position.1 as usize][adjacent_position.0 as usize].kind {
                                &DungeonCellKind::Passage | &DungeonCellKind::Room => true,
                                _ => false,
                            }
                        }
                        None => false,
                    };
                    if is_wall {
                        matrix[yu][xu].kind = DungeonCellKind::Wall;
                        break;
                    }
                }
            }
        }
    
        Self {
            matrix,
            spaces: new_spaces,
        }
    }
    pub fn get_size(&self) -> RectangleSize {
        (self.matrix[0].len() as u32, self.matrix.len() as u32)
    }
    pub fn find_cell(&self, position: &DungeonCellPosition) -> Option<&DungeonCell> {
        let size = self.get_size();
        if (0..size.0).contains(&position.0) && (0..size.1).contains(&position.1) {
            Some(&self.matrix[position.1 as usize][position.0 as usize])
        } else {
            None
        }
    }
    pub fn get_related_space_from_cell(&self, cell_position: &DungeonCellPosition) -> Option<&DungeonSpace> {
        let cell = self.find_cell(cell_position).unwrap();
        match cell.related_space_position {
            Some(related_space_position) => self.spaces.iter().find(|space| space.position == related_space_position),
            None => None,
        }
    }
    /// Output an overview of this dungeon for automated testing.
    pub fn to_text(&self) -> String {
        self.matrix.iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        match cell.kind {
                            DungeonCellKind::Blank => " ",
                            DungeonCellKind::Entrance => "+",
                            DungeonCellKind::Passage => "*",
                            DungeonCellKind::Room => ".",
                            DungeonCellKind::Wall => "#",
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    /// Output an overview of this dungeon for manual debugging.
    pub fn to_text_colored_with_ansi(&self) -> String {
        self.matrix.iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        let space = self.get_related_space_from_cell(&cell.position);
                        match cell.kind {
                            DungeonCellKind::Blank => String::from(" "),
                            DungeonCellKind::Entrance => format!("{}+{}", termion::color::Fg(termion::color::Magenta), termion::style::Reset),
                            DungeonCellKind::Passage => {
                                let space = space.unwrap();
                                let depth_string = space.depth.to_string().pop().unwrap().to_string();
                                format!("{}{}{}", termion::color::Fg(termion::color::Green), depth_string, termion::style::Reset)
                            },
                            DungeonCellKind::Room => {
                                let space = space.unwrap();
                                space.depth.to_string().pop().unwrap().to_string()
                            },
                            DungeonCellKind::Wall => format!("{}#{}", termion::color::Fg(termion::color::Yellow), termion::style::Reset),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests_of_dungeon {
    use super::*;

    fn create_test_instance() -> Dungeon {
        Dungeon {
            matrix: vec![],
            spaces: vec![],
        }
    }

    fn create_test_space() -> DungeonSpace {
        DungeonSpace {
            position: (0, 0),
            size: (1, 1),
            kind: DungeonSpaceKind::Room,
            depth: 0,
        }
    }

    mod tests_of_new {
        use super::*;
    
        fn create_test_space() -> DungeonSpace {
            DungeonSpace {
                position: (0, 0),
                size: (1, 1),
                kind: DungeonSpaceKind::Room,
                depth: 0,
            }
        }
    
        #[test]
        fn it_should_associate_cells_and_spaces() {
            let dungeon = Dungeon::new(
                &(3, 2),
                &vec![
                    DungeonSpace {
                        position: (0, 0),
                        size: (2, 2),
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (2, 1),
                        size: (1, 1),
                        ..create_test_space()
                    },
                ],
                0,
            );
            assert_eq!(&dungeon.matrix[0][0].related_space_position.unwrap(), &(0, 0));
            assert_eq!(&dungeon.matrix[0][1].related_space_position.unwrap(), &(0, 0));
            assert_eq!(dungeon.matrix[0][2].related_space_position.is_none(), true);
            assert_eq!(&dungeon.matrix[1][0].related_space_position.unwrap(), &(0, 0));
            assert_eq!(&dungeon.matrix[1][1].related_space_position.unwrap(), &(0, 0));
            assert_eq!(&dungeon.matrix[1][2].related_space_position.unwrap(), &(2, 1));
        }
        #[test]
        fn it_can_create_1x1_size() {
            let dungeon = Dungeon::new(
                &(1, 1),
                &vec![
                    DungeonSpace {
                        position: (0, 0),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Room,
                        ..create_test_space()
                    },
                ],
                0,
            );
            assert_eq!(
                dungeon.to_text(),
                vec![
                    ".",
                ].join("\n").to_string(),
            );
        }
        #[test]
        fn it_can_create_1x1_size_with_margin() {
            let dungeon = Dungeon::new(
                &(1, 1),
                &vec![
                    DungeonSpace {
                        position: (0, 0),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Room,
                        ..create_test_space()
                    },
                ],
                1,
            );
            assert_eq!(
                dungeon.to_text(),
                vec![
                    "###",
                    "#.#",
                    "###",
                ].join("\n").to_string(),
            );
        }
        #[test]
        fn it_should_enclose_room_and_passage_with_walls() {
            let dungeon = Dungeon::new(
                &(7, 4),
                &vec![
                    DungeonSpace {
                        position: (1, 1),
                        size: (2, 2),
                        kind: DungeonSpaceKind::Room,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (5, 1),
                        size: (1, 2),
                        kind: DungeonSpaceKind::Passage,
                        ..create_test_space()
                    },
                ],
                0,
            );
            assert_eq!(
                dungeon.to_text(),
                vec![
                    "#######",
                    "#..##*#",
                    "#..##*#",
                    "#######",
                ].join("\n").to_string(),
            );
        }
        #[test]
        fn it_should_not_enclose_entrance_with_walls() {
            let dungeon = Dungeon::new(
                &(3, 3),
                &vec![
                    DungeonSpace {
                        position: (1, 1),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Entrance,
                        ..create_test_space()
                    },
                ],
                0,
            );
            assert_eq!(
                dungeon.to_text(),
                vec![
                    "   ",
                    " + ",
                    "   ",
                ].join("\n").to_string(),
            );
        }
        #[test]
        fn it_can_create_a_matrix_including_all_kind_of_spaces() {
            let dungeon = Dungeon::new(
                &(9, 9),
                &vec![
                    DungeonSpace {
                        position: (3, 3),
                        size: (3, 3),
                        kind: DungeonSpaceKind::Room,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (4, 0),
                        size: (1, 2),
                        kind: DungeonSpaceKind::Passage,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (7, 4),
                        size: (2, 1),
                        kind: DungeonSpaceKind::Passage,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (4, 7),
                        size: (1, 2),
                        kind: DungeonSpaceKind::Passage,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (0, 4),
                        size: (2, 1),
                        kind: DungeonSpaceKind::Passage,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (4, 2),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Entrance,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (6, 4),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Entrance,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (4, 6),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Entrance,
                        ..create_test_space()
                    },
                    DungeonSpace {
                        position: (2, 4),
                        size: (1, 1),
                        kind: DungeonSpaceKind::Entrance,
                        ..create_test_space()
                    },
                ],
                0,
            );
            assert_eq!(
                dungeon.to_text(),
                vec![
                    "   #*#   ",
                    "   #*#   ",
                    "  ##+##  ",
                    "###...###",
                    "**+...+**",
                    "###...###",
                    "  ##+##  ",
                    "   #*#   ",
                    "   #*#   ",
                ].join("\n").to_string(),
            );
        }
    }

    mod tests_of_get_size {
        use super::*;

        #[test]
        fn it_works() {
            let instance = Dungeon {
                matrix: create_matrix(&(3, 2)),
                ..create_test_instance()
            };
            assert_eq!(instance.get_size(), (3, 2));
        }
    }

    mod tests_of_find_cell {
        use super::*;

        #[test]
        fn it_finds_the_top_left_cell() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(3, 2)),
                ..create_test_instance()
            };
            assert_eq!(
                &dungeon.find_cell(&(0, 0)).unwrap().position,
                &(0, 0),
            );
        }
        #[test]
        fn it_finds_the_bottom_right_cell() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(3, 2)),
                ..create_test_instance()
            };
            assert_eq!(
                &dungeon.find_cell(&(2, 1)).unwrap().position,
                &(2, 1),
            );
        }
        #[test]
        fn it_returns_none_when_the_position_exceeds_1_cell_to_the_right() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(3, 2)),
                ..create_test_instance()
            };
            assert_eq!(
                dungeon.find_cell(&(3, 1)).is_none(),
                true,
            );
        }
        #[test]
        fn it_returns_none_when_the_position_exceeds_1_cell_to_the_bottom() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(3, 2)),
                ..create_test_instance()
            };
            assert_eq!(
                dungeon.find_cell(&(2, 2)).is_none(),
                true,
            );
        }
    }

    mod tests_of_get_related_space_from_cell {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_when_it_specifies_the_outside_of_the_matrix() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(1, 1)),
                ..create_test_instance()
            };
            dungeon.get_related_space_from_cell(&(1, 0));
        }
        #[test]
        fn it_returns_none_when_the_cell_is_not_related_any_space() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(1, 1)),
                ..create_test_instance()
            };
            assert_eq!(
                dungeon.get_related_space_from_cell(&(0, 0)).is_none(),
                true,
            );
        }
        #[test]
        fn it_can_returns_a_related_space() {
            let dungeon = Dungeon {
                matrix: create_matrix(&(2, 2)).into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|cell| DungeonCell {
                                related_space_position: Some((0, 0)),
                                ..cell
                            })
                            .collect()
                    })
                    .collect(),
                spaces: vec![
                    DungeonSpace {
                        position: (0, 0),
                        size: (2, 2),
                        ..create_test_space()
                    },
                ],
                ..create_test_instance()
            };
            assert_eq!(
                &dungeon.get_related_space_from_cell(&(1, 1)).unwrap().position,
                &(0, 0),
            );
        }
    }

    mod tests_of_to_text {
        use super::*;

        #[test]
        fn it_works() {
            let mut matrix = create_matrix(&(3, 2));
            matrix[0][1].kind = DungeonCellKind::Entrance;
            matrix[0][2].kind = DungeonCellKind::Passage;
            matrix[1][0].kind = DungeonCellKind::Room;
            matrix[1][1].kind = DungeonCellKind::Wall;
            let instance = Dungeon {
                matrix,
                ..create_test_instance()
            };
            assert_eq!(
                instance.to_text(),
                " +*\n.# "
            );
        }
    }
}

fn create_candidate_positions_of_adjacent_space(
    dungeon_size: &RectangleSize,
    space_size: &RectangleSize,
    entrance_position: &DungeonCellPosition,
    direction_from_entrance: &FourDirection,
) -> Vec<DungeonCellPosition> {
    let entrance_xy: XYCoordinates = (entrance_position.0 as i32, entrance_position.1 as i32);
    let mut room_positions_as_xy: Vec<XYCoordinates> = vec![];
    match direction_from_entrance {
        FourDirection::Up => {
            for x in (entrance_xy.0 - space_size.0 as i32 + 1)..(entrance_xy.0 + 1) {
                room_positions_as_xy.push((x, entrance_xy.1 - space_size.1 as i32));
            };
        },
        FourDirection::Right => {
            for y in (entrance_xy.1 - space_size.1 as i32 + 1)..(entrance_xy.1 + 1) {
                room_positions_as_xy.push((entrance_xy.0 + 1, y));
            };
        },
        FourDirection::Down => {
            for x in (entrance_xy.0 - space_size.0 as i32 + 1)..(entrance_xy.0 + 1) {
                room_positions_as_xy.push((x, entrance_xy.1 + 1));
            };
        },
        FourDirection::Left => {
            for y in (entrance_xy.1 - space_size.1 as i32 + 1)..(entrance_xy.1 + 1) {
                room_positions_as_xy.push((entrance_xy.0 - space_size.0 as i32, y));
            };
        },
    };
    room_positions_as_xy.iter()
        .filter_map(|xy| {
            match relocate_xy_to_position(dungeon_size, xy) {
                Some(position) => {
                    if position.0 + space_size.0 <= dungeon_size.0 && position.1 + space_size.1 <= dungeon_size.1 {
                        Some(position)
                    } else {
                        None
                    }
                },
                None => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests_of_create_candidate_positions_of_adjacent_space {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        args: (RectangleSize, RectangleSize, DungeonCellPosition, FourDirection),
        expected: Vec<DungeonCellPosition>,
        name: String,
    }

    #[test]
    fn it_works() {
        let table: Vec::<TestCase> = vec![
            //  01234567
            // 0
            // 1  X-X-+
            // 2  +-+-+
            // 3    E
            TestCase {
                name: String::from("it enumerates candidates in the up direction"),
                args: (
                    (99, 99),
                    (3, 2),
                    (4, 3),
                    FourDirection::Up,
                ),
                expected: vec![
                    (2, 1),
                    (3, 1),
                    (4, 1),
                ],
            },
            //  0123456
            // 0
            // 1   X-+
            // 2  EX-+
            // 3   +-+
            TestCase {
                name: String::from("it enumerates candidates in the right direction"),
                args: (
                    (99, 99),
                    (3, 2),
                    (2, 2),
                    FourDirection::Right,
                ),
                expected: vec![
                    (3, 1),
                    (3, 2),
                ],
            },
            //  0123456
            // 0
            // 1   E
            // 2 X-X-+
            // 3 +-+-+
            TestCase {
                name: String::from("it enumerates candidates in the down direction"),
                args: (
                    (99, 99),
                    (3, 2),
                    (3, 1),
                    FourDirection::Down,
                ),
                expected: vec![
                    (1, 2),
                    (2, 2),
                    (3, 2),
                ],
            },
            //  012345
            // 0
            // 1 X-+
            // 2 X-+E
            // 3 +-+
            TestCase {
                name: String::from("it enumerates candidates in the left direction"),
                args: (
                    (99, 99),
                    (3, 2),
                    (4, 2),
                    FourDirection::Left,
                ),
                expected: vec![
                    (1, 1),
                    (1, 2),
                ],
            },
            //  0123
            // 0
            // 1XX-+
            // 2++-+
            // 3 E
            TestCase {
                name: String::from("it does not generate candidates when the left direction is out of range"),
                args: (
                    (99, 99),
                    (3, 2),
                    (1, 3),
                    FourDirection::Up,
                ),
                expected: vec![
                    (0, 1),
                    (1, 1),
                ],
            },
            //  012345
            // 0  EX-+
            // 1   +-+
            TestCase {
                name: String::from("it does not generate candidates when the up direction is out of range"),
                args: (
                    (99, 99),
                    (3, 2),
                    (2, 0),
                    FourDirection::Right,
                ),
                expected: vec![
                    (3, 0),
                ],
            },
            //  012345
            // 0     #
            // 1   E #
            // 2 XX-+#
            // 3 ++-+#
            TestCase {
                name: String::from("it does not generate candidates when the right direction is out of range"),
                args: (
                    (5, 99),
                    (3, 2),
                    (3, 1),
                    FourDirection::Down,
                ),
                expected: vec![
                    (1, 2),
                    (2, 2),
                ],
            },
            //  012345
            // 0
            // 1 X-+
            // 2 +-+E
            // 3######
            TestCase {
                name: String::from("it does not generate candidates when the down direction is out of range"),
                args: (
                    (99, 3),
                    (3, 2),
                    (4, 2),
                    FourDirection::Left,
                ),
                expected: vec![
                    (1, 1),
                ],
            },
        ];
        for test_case in table {
            assert_eq!(
                create_candidate_positions_of_adjacent_space(
                    &test_case.args.0, &test_case.args.1, &test_case.args.2, &test_case.args.3),
                test_case.expected,
                "{}",
                test_case.name,
            );
        }
    }
}

/// Create spaces around starting space.
/// 
/// It returns a list including pairs of `vec![room, entrance]` or `vec![passage, entrance]`.
/// 
/// - `directions_to_generate` - Directions to create a room. In production, always pass 4 directions.
fn create_spaces_around(
    get_random: &GetRandom,
    dungeon_size: &RectangleSize,
    min_room_size: &RectangleSize,
    max_room_size: &RectangleSize,
    min_passage_length: u32,
    max_passage_length: u32,
    room_rate: f64,
    starting_space: &DungeonSpace,
    directions_to_generate: &Vec<FourDirection>,
) -> Vec<Vec<DungeonSpace>> {
    let mut pairs_of_new_spaces: Vec<Vec<DungeonSpace>> = vec![];

    // Loop the directions generating next space in random order.
    let directions_to_generate_indexes = rand_utils::create_shuffled_indexes(get_random, directions_to_generate.len());
    for directions_to_generate_index in directions_to_generate_indexes {
        let direction = &directions_to_generate[directions_to_generate_index];

        // Determine the position of the entrance.
        let entrance_candidates = starting_space.create_candidates_of_next_entrance_position(dungeon_size, direction);
        if entrance_candidates.len() > 0 {
            let entrance_position = entrance_candidates[rand_utils::choice_random_index(get_random, entrance_candidates.len())];

            //
            // Tyr to create either a room or a passage.
            //
            let (kind, size) = if get_random() < room_rate {
                (
                    DungeonSpaceKind::Room,
                    random_space_size(get_random, min_room_size, max_room_size),
                )
            } else {
                let kind = DungeonSpaceKind::Passage;
                if direction == &FourDirection::Up || direction == &FourDirection::Down {
                    (
                        kind,
                        random_space_size(get_random, &(1, min_passage_length), &(1, max_passage_length)),
                    )
                } else {
                    (
                        kind,
                        random_space_size(get_random, &(min_passage_length, 1), &(max_passage_length, 1)),
                    )
                }
            };
            let position_candidates = create_candidate_positions_of_adjacent_space(
                &dungeon_size, &size, &entrance_position, direction);
            if position_candidates.len() > 0 {
                pairs_of_new_spaces.push(vec![
                    DungeonSpace {
                        kind,
                        size,
                        position: position_candidates[rand_utils::choice_random_index(get_random, position_candidates.len())],
                        depth: starting_space.depth + 1,
                    },
                    DungeonSpace {
                        kind: DungeonSpaceKind::Entrance,
                        size: (1, 1),
                        position: entrance_position,
                        depth: starting_space.depth + 1,
                    },
                ]);
            }
        }
    }

    pairs_of_new_spaces
}

#[cfg(test)]
mod tests_of_create_spaces_around {
    use super::*;

    fn create_test_instance() -> DungeonSpace {
        DungeonSpace {
            kind: DungeonSpaceKind::Room,
            size: (5, 5),
            position: (50, 50),
            depth: 0,
        }
    }

    #[test]
    fn it_does_not_create_any_space_when_directions_to_generate_is_empty() {
        let starting_point = DungeonSpace {
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        let pairs = create_spaces_around(
            &get_random,
            &(99, 99),
            &(1, 1),
            &(1, 1),
            1,
            1,
            1.0,
            &starting_point,
            &vec![],
        );
        assert_eq!(pairs.len(), 0);
    }
    #[test]
    fn it_also_creates_an_entrance() {
        let starting_point = DungeonSpace {
            depth: 10,
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        let pairs = create_spaces_around(
            &get_random,
            &(99, 99),
            &(1, 1),
            &(1, 1),
            1,
            1,
            1.0,
            &starting_point,
            &vec![FourDirection::Up],
        );
        assert_eq!(pairs[0][1].kind, DungeonSpaceKind::Entrance);
    }
    #[test]
    fn it_creates_4_pairs_when_4_directions_are_set_and_spaces_do_not_collide_to_walls_of_dungeon() {
        let starting_point = DungeonSpace {
            size: (9, 9),
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..100 {
            let pairs = create_spaces_around(
                &get_random,
                &(99, 99),
                &(1, 1),
                &(1, 1),
                1,
                1,
                1.0,
                &starting_point,
                &create_four_directions(),
            );
            assert_eq!(pairs.len(), 4);
        }
    }
    #[test]
    fn it_should_create_a_passage_with_a_straight_line_from_the_starting_point() {
        let starting_point = DungeonSpace {
            position: (3, 3),
            size: (1, 1),
            ..create_test_instance()
        };
        let get_random: GetRandom = || { 0.0 };
        //  0123456
        // 0   P
        // 1   P
        // 2   E
        // 3PPESEPP
        // 4   E
        // 5   P
        // 6   P
        let pairs = create_spaces_around(
            &get_random,
            &(99, 99),
            &(1, 1),
            &(1, 1),
            2,
            2,
            0.0,
            &starting_point,
            &create_four_directions(),
        );
        assert_eq!(pairs.iter().filter(|pair| pair[0].kind == DungeonSpaceKind::Passage).count(), 4);
        assert_eq!(pairs.iter().filter(|pair| pair[0].size == (1, 2)).count(), 2);
        assert_eq!(pairs.iter().filter(|pair| pair[0].size == (2, 1)).count(), 2);
    }
    #[test]
    fn it_always_creates_rooms_when_room_rate_is_one() {
        let starting_point = DungeonSpace {
            size: (1, 1),
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..100 {
            let pairs = create_spaces_around(
                &get_random,
                &(99, 99),
                &(1, 1),
                &(1, 1),
                1,
                1,
                1.0,
                &starting_point,
                &create_four_directions(),
            );
            for pair in pairs {
                assert_eq!(pair[0].kind, DungeonSpaceKind::Room);
            }
        }
    }
    #[test]
    fn it_always_creates_passages_when_room_rate_is_zero() {
        let starting_point = DungeonSpace {
            size: (1, 1),
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..100 {
            let pairs = create_spaces_around(
                &get_random,
                &(99, 99),
                &(1, 1),
                &(1, 1),
                1,
                1,
                0.0,
                &starting_point,
                &create_four_directions(),
            );
            for pair in pairs {
                assert_eq!(pair[0].kind, DungeonSpaceKind::Passage);
            }
        }
    }
    #[test]
    fn it_does_not_create_any_space_when_there_is_no_width() {
        let starting_point = DungeonSpace {
            size: (1, 1),
            position: (0, 0),
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..1 {
            let pairs = create_spaces_around(
                &get_random,
                &(1, 1),
                &(1, 1),
                &(1, 1),
                1,
                1,
                1.0,
                &starting_point,
                &create_four_directions(),
            );
            assert_eq!(pairs.len(), 0);
        }
    }
    #[test]
    fn it_increments_the_depth_value() {
        let starting_point = DungeonSpace {
            depth: 10,
            ..create_test_instance()
        };
        let get_random: GetRandom = || { rand::random::<f64>() };
        let pairs = create_spaces_around(
            &get_random,
            &(99, 99),
            &(1, 1),
            &(1, 1),
            1,
            1,
            1.0,
            &starting_point,
            &vec![FourDirection::Up],
        );
        assert_eq!(pairs[0][0].depth, 11);
        assert_eq!(pairs[0][1].depth, 11);
    }
}

fn generate_starting_room(
    get_random: &GetRandom,
    dungeon_size: &RectangleSize,
    min_room_size: &RectangleSize,
    max_room_size: &RectangleSize,
) -> DungeonSpace {
    let starting_room_size = random_space_size(get_random, min_room_size, max_room_size);
    let starting_room_position_candidates =
        create_candidates_of_inner_rectangle_position(dungeon_size, &starting_room_size);
    let starting_room_position = starting_room_position_candidates[
        rand_utils::choice_random_index(get_random, starting_room_position_candidates.len())
    ];
    DungeonSpace {
        kind: DungeonSpaceKind::Room,
        size: starting_room_size,
        position: starting_room_position,
        depth: 0,
    }
}

#[cfg(test)]
mod tests_generate_starting_room {
    use super::*;

    #[test]
    fn it_creates_the_same_size_when_min_and_max_size_are_the_same() {
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..100 {
            let space = generate_starting_room(&get_random, &(99, 99), &(3, 2), &(3, 2));
            assert_eq!(space.size, (3, 2));
        }
    }
}

fn generate_spaces(
    get_random: &GetRandom,
    dungeon_size: &RectangleSize,
    min_room_size: &RectangleSize,
    max_room_size: &RectangleSize,
    min_passage_length: u32,
    max_passage_length: u32,
    room_rate: f64,
    starting_room: DungeonSpace,
) -> Vec<DungeonSpace> {
    // NOTE: The length of the wall cannot be changed.
    //       There are other processes that is depending in the 1 wall length.
    let wall_length: u32 = 1;

    let mut spaces: Vec<DungeonSpace> = vec![];

    //
    // Place the starting room.
    //
    spaces.push(starting_room);

    //
    // Create spaces step by step from the starting room.
    //
    // NOTE: No recursive processing. This is because some branches grow particularly long.
    //
    let mut computed_space_positions: Vec<DungeonCellPosition> = vec![];
    let mut placed_locations_including_walls: Vec<(XYCoordinates, RectangleSize)> = vec![
        spaces[0].get_location_as_xy(wall_length),
    ];
    let four_directions = create_four_directions();
    loop {
        let unprocessed_spaces: Vec<&DungeonSpace> = spaces.iter()
            .filter(|space| {
                (space.kind == DungeonSpaceKind::Room || space.kind == DungeonSpaceKind::Passage) &&
                !computed_space_positions.contains(&space.position)
            })
            .collect();
        if unprocessed_spaces.len() == 0 {
            break;
        }
        let mut new_spaces_in_current_loop: Vec<DungeonSpace> = vec![];
        for unprocessed_space in unprocessed_spaces {
            let mut pairs = create_spaces_around(
                get_random,
                dungeon_size,
                min_room_size,
                max_room_size,
                min_passage_length,
                max_passage_length,
                room_rate,
                unprocessed_space,
                &four_directions,
            );
            // Place new spaces if they do not overlap with other spaces.
            pairs.retain(|pair| {
                let space = &pair[0];
                let entrance = &pair[1];
                // NOTE: Ignore the overlapping check to each entrance.
                //       Because it is always buried in the wall of the connected room or passage.
                let is_overlapping = space.is_overlapping_to_others(0, &placed_locations_including_walls);
                if !is_overlapping {
                    placed_locations_including_walls.push(space.get_location_as_xy(wall_length));
                    placed_locations_including_walls.push(entrance.get_location_as_xy(wall_length));
                }
                !is_overlapping
            });
            let mut new_spaces: Vec<DungeonSpace> = pairs.into_iter().flatten().collect();
            new_spaces_in_current_loop.append(&mut new_spaces);
            computed_space_positions.push(unprocessed_space.position);
        }
        spaces.append(&mut new_spaces_in_current_loop);
    }

    spaces
}

#[cfg(test)]
mod tests_of_generate_spaces {
    use super::*;

    fn create_test_starting_room() -> DungeonSpace {
        DungeonSpace {
            position: (0, 0),
            size: (1, 1),
            kind: DungeonSpaceKind::Room,
            depth: 0,
        }
    }

    #[test]
    fn it_should_include_only_the_starting_room_when_dungeon_is_filled_with_it() {
        let get_random: GetRandom = || { rand::random::<f64>() };
        let spaces = generate_spaces(
            &get_random,
            &(3, 2),
            &(3, 2),
            &(3, 2),
            1,
            1,
            0.0,
            DungeonSpace {
                position: (0, 0),
                size: (3, 2),
                ..create_test_starting_room()
            },
        );
        assert_eq!(spaces.len(), 1);
    }
    #[test]
    fn it_should_create_plural_rooms_when_the_dungeon_can_store_spaces() {
        #[derive(Debug)]
        struct TestParameters {
            dungeon_size: RectangleSize,
            starting_room_position: DungeonCellPosition,
        }
        let get_random: GetRandom = || { rand::random::<f64>() };
        let test_data = [
            //  01234
            // 0SERER
            TestParameters {
                dungeon_size: (5, 1),
                starting_room_position: (0, 0),
            },
            //  01234
            // 0RESER
            TestParameters {
                dungeon_size: (5, 1),
                starting_room_position: (2, 0),
            },
            //  01234
            // 0RERES
            TestParameters {
                dungeon_size: (5, 1),
                starting_room_position: (4, 0),
            },
            //  0
            // 0S
            // 1E
            // 2R
            // 3E
            // 4R
            TestParameters {
                dungeon_size: (1, 5),
                starting_room_position: (0, 0),
            },
            //  0
            // 0R
            // 1E
            // 2S
            // 3E
            // 4R
            TestParameters {
                dungeon_size: (1, 5),
                starting_room_position: (0, 2),
            },
            //  0
            // 0R
            // 1E
            // 2R
            // 3E
            // 4S
            TestParameters {
                dungeon_size: (1, 5),
                starting_room_position: (0, 4),
            },
        ];
        for test_parameters in test_data {
            for _ in 0..100 {
                let spaces = generate_spaces(
                    &get_random,
                    &test_parameters.dungeon_size,
                    &(1, 1),
                    &(1, 1),
                    1,
                    1,
                    1.0,
                    DungeonSpace {
                        position: test_parameters.starting_room_position,
                        size: (1, 1),
                        ..create_test_starting_room()
                    },
                );
                assert_eq!(spaces.len(), 5, "Failed in the {:?} and {:?}.", test_parameters, spaces);
                assert_eq!(spaces.iter().filter(|e| e.kind == DungeonSpaceKind::Room).count(), 3, "Failed in the {:?} and {:?}.", test_parameters, spaces);
                assert_eq!(spaces.iter().filter(|e| e.kind == DungeonSpaceKind::Entrance).count(), 2, "Failed in the {:?} and {:?}.", test_parameters, spaces);
            }
        }
    }
}

#[derive(Debug)]
pub struct DungeonGenerationParameters {
    pub dungeon_size: RectangleSize,
    pub max_passage_length: u32,
    pub max_room_size: RectangleSize,
    pub min_passage_length: u32,
    pub min_room_size: RectangleSize,
    /// The range where it can not generate spaces but can generate walls.
    /// 
    /// Actually, you will set either `0` or `1`.
    pub padding_length: u32,
    pub room_rate: f64,
}
impl Default for DungeonGenerationParameters {
    fn default() -> Self {
        Self {
            dungeon_size: (80, 24),
            min_room_size: (3, 3),
            max_room_size: (9, 9),
            min_passage_length: 1,
            max_passage_length: 9,
            padding_length: 1,
            room_rate: 0.5,
        }
    }
}

// TODO: アルゴリズム的にはstarting_pointからの袋小路状になるが、調整を入れて枝先同士で結合して巡回できるようにしたい。
// TODO: 枝の末端が通路で終わると、移動が徒労になりそうでよくなさそう？　通路-通路-通路(末端) 的なのが見た目が悪いという面もある。
pub fn generate_dungeon(get_random: &GetRandom, parameters: &DungeonGenerationParameters) -> Dungeon {
    let real_dungeon_size: RectangleSize = (
        parameters.dungeon_size.0 - parameters.padding_length as u32 * 2,
        parameters.dungeon_size.1 - parameters.padding_length as u32 * 2,
    );
    let starting_room = generate_starting_room(
        get_random,
        &real_dungeon_size,
        &parameters.min_room_size,
        &parameters.max_room_size,
    );
    let spaces = generate_spaces(
        get_random,
        &real_dungeon_size,
        &parameters.min_room_size,
        &parameters.max_room_size,
        parameters.min_passage_length,
        parameters.max_passage_length,
        parameters.room_rate,
        starting_room,
    );
    Dungeon::new(
        &real_dungeon_size,
        &spaces,
        parameters.padding_length,
    )
}

#[cfg(test)]
mod tests_of_generate_dungeon {
    use super::*;

    #[test]
    fn output_dungeon_overview_for_manual_debugging() {
        let get_random: GetRandom = || { rand::random::<f64>() };
        let params = DungeonGenerationParameters {
            ..Default::default()
        };
        let dungeon = generate_dungeon(&get_random, &params);
        println!("{}", dungeon.to_text_colored_with_ansi());
    }

    #[test]
    fn it_does_not_panic_at_least_when_it_runs_100_times() {
        let get_random: GetRandom = || { rand::random::<f64>() };
        for _ in 0..100 {
            let params = DungeonGenerationParameters {
                ..Default::default()
            };
            generate_dungeon(&get_random, &params);
        }
    }
}
