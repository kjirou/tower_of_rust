extern crate clap;
extern crate termion;
extern crate tower_of_rust;

use clap::{Arg, App};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use termion::{clear, cursor, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tower_of_rust::controller::Controller;

fn main() {
    let command_args = App::new("A Tower of Rust")
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .short("d")
                .help("Don't run the TUI application for debugging with print functions.")
        )
        .get_matches();

    let mut controller = Controller::new();
    
    if command_args.is_present("debug") {
        let output = controller.create_screen_output_as_lines().join("\n");
        println!("{}", output);
    } else {
        let (tx, rx) = mpsc::channel::<Key>();

        let main_loop_handle = thread::spawn(move || {
            // NOTE: Restore the state of the previous terminal when dropping `stdout` variable.
            //       https://github.com/redox-os/termion/blob/dce5e7500fd709987f9bf8f3911e4daa61d0ad14/src/raw.rs#L34-L37
            //       If an user exits this program without dropping `stdout`, the user's terminal will break.
            let mut stdout = io::stdout().into_raw_mode().unwrap();

            write!(stdout, "{}{}", cursor::Hide, clear::All).unwrap();
            stdout.flush().unwrap();

            let mut previous_output_lines: [String; 24] = Default::default();

            loop {
                let now = Instant::now();
                let key_input = match rx.try_recv() {
                    Ok(key_input) => Some(key_input),
                    Err(_) => None,
                };

                // Purge extra key inputs in the same frame.
                while rx.try_recv().is_err() == false {};

                // Quit this application. Only this operation is resolved with priority.
                match key_input {
                    Some(key_input) => {
                        match key_input {
                            Key::Esc | Key::Ctrl('c') => break,
                            _ => {},
                        }
                    },
                    _ => {},
                };

                controller.handle_main_roop(&now, key_input);

                for (y, line) in controller.create_screen_output_as_lines().iter().enumerate() {
                    if &previous_output_lines[y] != line {
                        write!(stdout, "{}{}", cursor::Goto(1, y as u16 + 1), line).unwrap();
                        previous_output_lines[y] = String::from(line);
                    }
                }
                write!(stdout, "{}", style::Reset).unwrap();
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
                // Quit this application.
                Key::Esc | Key::Ctrl('c') => {
                    tx.send(key_input).unwrap();
                    break;
                },
                Key::Up | Key::Right | Key::Down | Key::Left => {
                    tx.send(key_input).unwrap();
                },
                Key::Char(key_input) => tx.send(Key::Char(key_input)).unwrap(),
                _ => {},
            };
        }

        // NOTE: Must wait here to unlock Row Mode and reset ANSI.
        main_loop_handle.join().unwrap();
    }
}
