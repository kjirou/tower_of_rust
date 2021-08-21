extern crate clap;
extern crate termion;

extern crate tower_of_rust;

use clap::{Arg, App};

use termion::{clear, cursor, style};
use tower_of_rust::models::field::Field;
use tower_of_rust::screen::Screen;

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

    // TODO: screen と models を直接参照させない。間に React の Props みたいな更新クエリの概念を挟む。
    for (y, row_of_field_element) in field.matrix.iter().enumerate() {
        for (x, field_element) in row_of_field_element.iter().enumerate() {
            // TODO: FieldElement の位置と Screen の位置が同じになるとは限らない。というか、Field の方が Screen の Field 描画範囲より大きい。
            screen.matrix[y][x].symbol = field_element.get_display();
        }
    }

    let output = screen.matrix.iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.symbol.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    
    if command_args.is_present("debug") {
        println!("{}", output);
    } else {
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
