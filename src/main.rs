extern crate tower_of_rust;

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

    println!("{}", output);
}
