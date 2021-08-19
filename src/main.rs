extern crate termion;

extern crate tower_of_rust;

use termion::{clear, cursor, style};
use tower_of_rust::models::field::Field;
use tower_of_rust::screen::Screen;

fn main() {
    let screen = Screen::create_instance();

    let output = screen.matrix.iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.symbol.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("\n{}{}{}{}",
        cursor::Hide,
        clear::All,
        cursor::Goto(1, 1),
        output);
    println!("{}{}",
        style::Reset,
        cursor::Show);

    // TODO: For debugging.
    let field = Field::new(3, 2);
    println!("{:?}", field);
}
