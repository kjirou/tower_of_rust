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
    screen.update(&field);
    
    if command_args.is_present("debug") {
        let output = screen.create_output_as_lines().join("\n");
        println!("{}", output);
    } else {
        let (tx, rx): (std::sync::mpsc::Sender<Key>, std::sync::mpsc::Receiver<Key>) = mpsc::channel();

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
