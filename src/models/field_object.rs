use std::cmp;

const MOVEMENT_POWER_PER_STEP: u16 = 216;
const MAX_MOVEMENT_POWER: u16 = MOVEMENT_POWER_PER_STEP * 4;

#[derive(Debug)]
pub enum DisplayKind {
    Hero,
    Wall,
}

#[derive(Debug)]
pub struct FieldObject {
    pub display_kind: DisplayKind,
    pub id: String,
    pub is_obstacle: bool,
    pub mobility: u16,
    pub movement_power: u16,
}
impl FieldObject {
    pub fn new_hero(id: &str) -> Self {
        Self {
            id: id.to_string(),
            display_kind: DisplayKind::Hero,
            is_obstacle: true,
            mobility: 72,
            movement_power: 0,
        }
    }
    pub fn new_wall(id: &str) -> Self {
        Self {
            id: id.to_string(),
            display_kind: DisplayKind::Wall,
            is_obstacle: true,
            mobility: 0,
            movement_power: 0,
        }
    }
    pub fn can_move(&self) -> bool {
        self.mobility > 0
    }
    pub fn can_step(&self) -> bool {
        self.movement_power >= MOVEMENT_POWER_PER_STEP
    }
    pub fn accumulate_movement_power(&mut self) {
        self.movement_power = cmp::min(self.movement_power + self.mobility, MAX_MOVEMENT_POWER);
    }
    pub fn consume_movement_power_for_step(&mut self) {
        if !self.can_step() {
            panic!("It is not accumulating mobility.");
        }
        self.movement_power = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_creature(id: &str) -> FieldObject {
        FieldObject {
            id: id.to_string(),
            display_kind: DisplayKind::Hero,
            is_obstacle: true,
            mobility: 1,
            movement_power: 0,
        }
    }

    mod tests_of_accumulate_movement_power {
        use super::*;

        #[test]
        fn it_works() {
            let mut creature = FieldObject {
                mobility: MOVEMENT_POWER_PER_STEP,
                ..new_test_creature("a")
            };
            for _ in 0..4 {
                creature.accumulate_movement_power();
            }
            assert_eq!(creature.movement_power, MOVEMENT_POWER_PER_STEP * 4);
            creature.accumulate_movement_power();
            assert_eq!(creature.movement_power, MOVEMENT_POWER_PER_STEP * 4);  // Not updated.
        }
    }
}
