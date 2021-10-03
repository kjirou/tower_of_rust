use std::time::{Duration, Instant};
use termion::event::Key;

use crate::types::FieldObjectLocation;

#[derive(Debug)]
pub struct Game {
    last_fps_calculation: Option<(Instant, u64, f64)>,
    pub last_key_input: Option<Key>,
    pub number_of_frames: u64,
    pub operation_target_location: Option<FieldObjectLocation>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            number_of_frames: 0,
            last_fps_calculation: None,
            last_key_input: None,
            operation_target_location: None,
        }
    }
    pub fn get_fps(&self) -> f64 {
        match self.last_fps_calculation {
            Some(last_key_input) => last_key_input.2,
            None => 0.0,
        }
    }
    /// Caluculate the FPS in 3-second cycles.
    /// 
    /// This calculation is a bit inaccurate.
    pub fn caluculate_fps_in_3_second_cycles(&mut self, now: &Instant) {
        let caluculation_cycle: u64 = 3;  // sec
        match self.last_fps_calculation {
            Some(last_fps_calculation) => {
                // NOTE: Since the decision is made in more than 3 seconds, it may exceed 3 seconds.
                if now.duration_since(last_fps_calculation.0) >= Duration::from_secs(caluculation_cycle) {
                    let fps = (self.number_of_frames - last_fps_calculation.1) as f64 / caluculation_cycle as f64;
                    self.last_fps_calculation = Some((now.clone(), self.number_of_frames, fps));
                }
            },
            None => {
                self.last_fps_calculation = Some((now.clone(), self.number_of_frames, 0.0));
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_instance() -> Game {
        Game::new()
    }

    mod tests_of_caluculate_fps_in_3_second_cycles {
        use super::*;

        #[test]
        fn it_should_always_update_the_fps_to_zero_at_the_first_call() {
            let mut instance = create_test_instance();
            let started = Instant::now();
            instance.caluculate_fps_in_3_second_cycles(&started);
            assert_eq!(instance.last_fps_calculation.unwrap().2, 0.0);
        }
        #[test]
        fn it_should_update_the_fps_with_a_call_after_3_seconds() {
            let mut instance = create_test_instance();
            let first_call = Instant::now();
            let second_call = first_call + Duration::from_millis(3000);
            let third_call = second_call + Duration::from_millis(2999);
            let fourth_call = third_call + Duration::from_millis(1);
            instance.caluculate_fps_in_3_second_cycles(&first_call);
            instance.number_of_frames = 90;
            instance.caluculate_fps_in_3_second_cycles(&second_call);
            assert_eq!(instance.last_fps_calculation.unwrap().2, 30.0);  // Updated.
            instance.number_of_frames = 135;
            instance.caluculate_fps_in_3_second_cycles(&third_call);
            assert_eq!(instance.last_fps_calculation.unwrap().2, 30.0);  // Not updated.
            instance.caluculate_fps_in_3_second_cycles(&fourth_call);
            assert_eq!(instance.last_fps_calculation.unwrap().2, 15.0);  // Updated.
        }
    }
}
