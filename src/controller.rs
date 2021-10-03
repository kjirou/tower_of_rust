use rand;
use std::time::Instant;
use termion::event::Key;

use crate::enums::FourDirection;
use crate::models::field::Field;
use crate::models::field_object::FieldObject;
use crate::models::game::Game;
use crate::operations::*;
use crate::screen::Screen;
use crate::screen_update_builder;
use crate::types::{GetRandom, RectangleSize};
use crate::utils::dungeon_generator::{self, DungeonGenerationParameters, DungeonSpace, DungeonSpaceKind};
use crate::utils::rand_utils;

pub struct Controller {
    field: Field,
    game: Game,
    screen: Screen,
}

impl Controller {
    pub fn new() -> Self {
        // NOTE: There is no reason for it to match the size of the screen.
        let field_size: RectangleSize = (80, 24);

        let get_random: GetRandom = || { rand::random::<f64>() };

        let dungeon = dungeon_generator::generate_dungeon(&get_random, &DungeonGenerationParameters {
            dungeon_size: field_size,
            ..Default::default()
        });

        // Deside where to place the hero.
        let rooms: Vec<&DungeonSpace> = dungeon.spaces.iter().filter(|e| e.kind == DungeonSpaceKind::Room).collect();
        let room_where_hero_is_placed = rooms[rand_utils::choice_random_index(&get_random, rooms.len())];
        let position_where_hero_is_placed = room_where_hero_is_placed.get_random_position_in_space(&get_random);

        let mut field = Field::new(&field_size);
        field.import_dungeon(&dungeon);
        field.place_field_object(&position_where_hero_is_placed, FieldObject::new_hero("hero"));

        let mut game = Game::new();
        game.operation_target_location = Some((position_where_hero_is_placed, String::from("hero")));

        let mut screen = Screen::new();
        screen.update(&screen_update_builder::build(&field, &game));

        Self {
            field,
            game,
            screen,
        }
    }
    pub fn handle_main_roop(&mut self, now: &Instant, key_input: Option<Key>) {
        self.game.number_of_frames += 1;
        self.game.caluculate_fps_in_3_second_cycles(now);
        if key_input.is_some() {
            self.game.last_key_input = key_input;
        }

        // Operate the target.
        if let Some(key_input) = key_input {
            match key_input {
                Key::Up | Key::Char('k') => move_operation_target_for_one_step(&mut self.field, &mut self.game, &FourDirection::Up),
                Key::Right | Key::Char('l') => move_operation_target_for_one_step(&mut self.field, &mut self.game, &FourDirection::Right),
                Key::Down | Key::Char('j') => move_operation_target_for_one_step(&mut self.field, &mut self.game, &FourDirection::Down),
                Key::Left | Key::Char('h') => move_operation_target_for_one_step(&mut self.field, &mut self.game, &FourDirection::Left),
                _ => {},
            }
        }

        // Perform state changes over time.
        self.field.perform_state_changes_over_time();

        // Transfer changes in models to the view model.
        let screen_update = screen_update_builder::build(&self.field, &self.game);
        self.screen.update(&screen_update);
    }
    pub fn create_screen_output_as_lines(&self) -> Vec<String> {
        self.screen.create_output_as_lines()
    }
}
