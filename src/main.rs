extern crate clap;
extern crate termion;

extern crate tower_of_rust;

use clap::{Arg, App};
use termion::{clear, cursor, style};
use tower_of_rust::models::field::Field;
use tower_of_rust::screen::Screen;

fn create_output(screen: &Screen) -> String {
    screen.matrix.iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.symbol.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
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

    let mut field = Field::new(25, 9);
    field.surround_with_walls();

    let mut screen = Screen::new();
    screen.update(&field);
    
    if command_args.is_present("debug") {
        let output = create_output(&screen);
        println!("{}", output);
    } else {
        let output = create_output(&screen);
        println!("\n{}{}{}{}",
            cursor::Hide,
            clear::All,
            cursor::Goto(1, 1),
            output);
        println!("{}{}",
            style::Reset,
            cursor::Show);
    }
}
