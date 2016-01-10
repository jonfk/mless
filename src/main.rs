
extern crate rustbox;

use std::env;
use std::error::Error;
use std::default::Default;
use std::io::prelude::*;
use std::fs::File;

use rustbox::{Color, RustBox};
use rustbox::Key;


fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("Missing filename (\"less --help\" for help)");
        return;
    }

    // Only support 1 file for now
    let filename = &args[0];

    let contents = match open_file(filename) {
        Result::Ok(v) => v,
        Result::Err(e) => {
            println!("{}",e);
            return;
        },
    };

    let contents_line_length = contents.lines().count();


    // let rustbox = RustBox::init(Default::default()).unwrap();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => {
            panic!("{}", e);
        },
    };

    let mut position: i64 = 0;

    print_screen(&rustbox, &contents, position);


    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    Some(Key::Char('j')) => {
                        let height = rustbox.height();
                        if position > ((contents_line_length as i64) - (height as i64)) {
                            continue;
                        }
                        position += 1;
                        print_screen(&rustbox, &contents, position);
                    }
                    Some(Key::Char('k')) => {
                        if position == 0 {
                            continue;
                        }
                        position -= 1;
                        print_screen(&rustbox, &contents, position);
                    }
                    Some(Key::Char('G')) => {
                        let new_position = ((contents_line_length as i64) - (rustbox.height() as i64)) + 1;
                        if new_position < 0 {
                            continue;
                        }
                        position = new_position;
                        print_screen(&rustbox, &contents, position);
                    }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {
                unreachable!("rustbox poll_event");
            }
        }
    }
}

fn open_file(filename: &str) -> Result<String, String> {

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = try!(File::open(filename).map_err(|err| format!("couldn't open {}: {}", filename, Error::description(&err))));

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();

    try!(file.read_to_string(&mut contents).map_err(|err| format!("couldn't read {}: {}", filename, Error::description(&err))));
    Ok(contents)
}

fn print_screen(rustbox: &RustBox, contents: &String, position: i64) {
    rustbox.clear();
    for (i, line) in contents.lines().skip(position as usize).enumerate() {
        rustbox.print(0, i, rustbox::RB_NORMAL, Color::White, Color::Black, line);
    }

    rustbox.print(0, rustbox.height()-1, rustbox::RB_BOLD, Color::Black, Color::White,
                  &format!("Press 'q' to quit. Pos: {}", position));

    rustbox.present();
}
