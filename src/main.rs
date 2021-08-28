extern crate clap;
extern crate termion;
extern crate tower_of_rust;

use clap::{Arg, App};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::{clear, cursor, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tower_of_rust::models::field::Field;
use tower_of_rust::models::field::FieldElementPosition;
use tower_of_rust::models::field_object::FieldObject;
use tower_of_rust::models::game::Game;
use tower_of_rust::screen::Screen;
use tower_of_rust::screen_update::MapElementUpdate;
use tower_of_rust::screen_update::ScreenUpdate;

//
// Reducers
//

fn move_operation_target(game: &Game, field: &mut Field, to: &FieldElementPosition) {
    match &game.operation_target {
        Some(operation_target) => field.move_field_object(operation_target, to),
        None => {
            panic!("There is no operation target.");
        },
    };
}

//
// Controller
//

fn create_screen_update(field: &Field) -> ScreenUpdate {
    let map_size = (21, 13);  // width, height
    let mut map: Vec<Vec<MapElementUpdate>> = vec![];

    for map_y in 0..map_size.1 {
        let mut map_row: Vec<MapElementUpdate> = vec![];
        for map_x in 0..map_size.0 {
            // TODO: Hero 表示位置が常に Map 中央になるように調整する。
            // TODO: Field の範囲を超えた時に、何かで埋める。
            let xy = (map_x, map_y);
            let field_element = field.get_field_element(&xy);
            let symbol = field_element.get_display();
            map_row.push(MapElementUpdate {
                symbol,
                foreground: String::from(""),
                background: String::from(""),
            });
        }
        map.push(map_row);
    }

    ScreenUpdate {
        map,
    }
}

fn main() {
    let command_args = App::new("A Tower of Rust")
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .short("d")
                .help("Don't run the TUI application for debugging with print functions.")
        )
        .get_matches();

    let mut game = Game {
        operation_target: None,
    };
    let mut field = Field::new(120, 36);
    field.surround_with_walls();
    field.place_field_object(&(2, 2), FieldObject::new_hero(String::from("player")));
    game.operation_target = Some((2, 2, String::from("player")));
    move_operation_target(&game, &mut field, &(2, 3));

    let mut screen = Screen::new();
    screen.update(&create_screen_update(&field));
    
    if command_args.is_present("debug") {
        let output = screen.create_output_as_lines().join("\n");
        println!("{}", output);
    } else {
        let (tx, rx) = mpsc::channel::<Key>();

        let main_loop_handle = thread::spawn(move || {
            // NOTE: Restores the state of the previous terminal when dropping `stdout` variable.
            //       https://github.com/redox-os/termion/blob/dce5e7500fd709987f9bf8f3911e4daa61d0ad14/src/raw.rs#L34-L37
            //       If user exits the program without dropping, the terminal will break.
            let mut stdout = io::stdout().into_raw_mode().unwrap();

            write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();
            stdout.flush().unwrap();

            loop {
                match rx.try_recv() {
                    Ok(key_input) => {
                        // TODO: For debug.
                        print!("{:?}", key_input);
                        match key_input {
                            Key::Esc | Key::Ctrl('c') | Key::Char('q') => {
                                break;
                            },
                            _ => {},
                        };
                    },
                    Err(_) => {},
                };

                // Purge extra key inputs in the same frame.
                while rx.try_recv().is_err() == false {};

                screen.update(&create_screen_update(&field));

                for (i, line) in screen.create_output_as_lines().iter().enumerate() {
                    write!(stdout, "{}{}", cursor::Goto(1, i as u16 + 1), line).unwrap();
                }
                stdout.flush().unwrap();

                thread::sleep(Duration::from_millis(33));
            }

            write!(stdout, "{}{}", style::Reset, cursor::Show).unwrap();
            stdout.flush().unwrap();
        });

        let stdin = io::stdin();
        for key_input in stdin.keys() {
            let key_input = key_input.unwrap();
            match key_input {
                Key::Esc | Key::Ctrl('c') | Key::Char('q') => {
                    tx.send(key_input).unwrap();
                    break;
                },
                Key::Char(key_input) => tx.send(Key::Char(key_input)).unwrap(),
                _ => {},
            };
        }

        // NOTE: Must wait here to unlock Row Mode and reset ANSI.
        main_loop_handle.join().unwrap();
    }
}
