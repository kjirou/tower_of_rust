use crate::actions::*;
use crate::enums::FourDirection;
use crate::models::field::Field;
use crate::models::field_object::FieldObject;
use crate::models::game::Game;
use crate::screen::Screen;
use crate::screen_update_builder;
use termion::event::Key;

pub struct Controller {
    field: Field,
    game: Game,
    screen: Screen,
}

impl Controller {
    pub fn handle_main_roop(&mut self, key_input: Option<Key>) {
        match key_input {
            Some(key_input) => {
                match key_input {
                    Key::Up | Key::Char('k') => move_hero(&mut self.field, &mut self.game, &FourDirection::Up),
                    Key::Right | Key::Char('l') => move_hero(&mut self.field, &mut self.game, &FourDirection::Right),
                    Key::Down | Key::Char('j') => move_hero(&mut self.field, &mut self.game, &FourDirection::Down),
                    Key::Left | Key::Char('h') => move_hero(&mut self.field, &mut self.game, &FourDirection::Left),
                    _ => advance_only_time(),
                };
            },
            None => advance_only_time(),
        };

        let screen_update = screen_update_builder::build(&self.field, &self.game);
        self.screen.update(&screen_update);
    }
    pub fn create_screen_output_as_lines(&self) -> Vec<String> {
        self.screen.create_output_as_lines()
    }
    pub fn new() -> Self {
        let mut field = Field::new(&(120, 36));
        field.surround_with_walls();
        field.place_field_object(&(2, 2), FieldObject::new_hero(String::from("player")));

        let mut game = Game {
            operation_target: None,
        };
        game.operation_target = Some(((2, 2), String::from("player")));

        let mut screen = Screen::new();
        screen.update(&screen_update_builder::build(&field, &game));

        Self {
            field,
            game,
            screen,
        }
    }
}
