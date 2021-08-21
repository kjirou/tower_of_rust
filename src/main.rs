extern crate termion;

extern crate tower_of_rust;

//use termion::{clear, cursor, style};
use tower_of_rust::models::field::Field;
use tower_of_rust::screen::Screen;

fn main() {
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
    
    println!("{}", output);
    // TODO: println でデバッグしやすいように、コマンドオプションで出力モードを変更する。
    // println!("\n{}{}{}{}",
    //     cursor::Hide,
    //     clear::All,
    //     cursor::Goto(1, 1),
    //     output);
    // println!("{}{}",
    //     style::Reset,
    //     cursor::Show);
}
