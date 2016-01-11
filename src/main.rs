
extern crate rustbox;

use std::env;
use std::error::Error;
use std::default::Default;
use std::io::prelude::*;
use std::fs::File;

use rustbox::{Color, RustBox};
use rustbox::Key;

mod test;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("Missing filename (\"less --help\" for help)");
        return;
    }

    // Only support 1 file for now
    let filename = &args[0];

    let orig_contents = match open_file(filename) {
        Result::Ok(v) => v,
        Result::Err(e) => {
            println!("{}", e);
            return;
        }
    };


    // let rustbox = RustBox::init(Default::default()).unwrap();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => {
            panic!("{}", e);
        }
    };

    let mut contents = word_wrap(rustbox.width(), orig_contents);

    let mut contents_line_length = contents.lines().count();

    let mut position: i64 = 0;

    let mut mini_buffer: MiniBuffer = MiniBuffer::new();
    mini_buffer.set_info(filename);

    print_screen(&rustbox, position, &contents, &mini_buffer);


    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) | Some(Key::Char('Q')) => {
                        break;
                    }
                    Some(Key::Char('j')) => {
                        let height = rustbox.height();
                        if position > ((contents_line_length as i64) - (height as i64)) {
                            continue;
                        }
                        position += 1;
                        mini_buffer.clear();
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    Some(Key::Char('k')) => {
                        if position == 0 {
                            continue;
                        }
                        position -= 1;
                        mini_buffer.clear();
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    Some(Key::Char('G')) => {
                        let new_position = ((contents_line_length as i64) -
                                            (rustbox.height() as i64)) +
                                           1;
                        if new_position < 0 {
                            continue;
                        }
                        position = new_position;
                        mini_buffer.clear();
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    Some(Key::Char('g')) => {
                        if mini_buffer.buffer.len() > 0 &&
                           mini_buffer.mode == MiniBufferMode::Normal {
                            position = mini_buffer.buffer
                                                  .iter()
                                                  .map(|c| *c)
                                                  .collect::<String>()
                                                  .parse::<i64>()
                                                  .unwrap_or(0);
                            mini_buffer.clear();
                        } else {
                            position = 0;
                        }
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    Some(Key::Char(key)) if key >= '0' && key <= '9' => {
                        mini_buffer.buffer.push(key);
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    Some(Key::Char('=')) |
                    Some(Key::Ctrl('g')) |
                    Some(Key::Ctrl('G')) |
                    Some(Key::Char('f')) => {
                        mini_buffer.set_info(filename);
                        print_screen(&rustbox, position, &contents, &mini_buffer);
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {
                unreachable!("rustbox poll_event");
            }
        }
    }
}

fn open_file(filename: &str) -> Result<String, String> {

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = try!(File::open(filename).map_err(|err| {
        format!("couldn't open {}: {}", filename, Error::description(&err))
    }));

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();

    try!(file.read_to_string(&mut contents)
             .map_err(|err| format!("couldn't read {}: {}", filename, Error::description(&err))));
    Ok(contents)
}

fn print_screen(rustbox: &RustBox, position: i64, contents: &String, mini_buffer: &MiniBuffer) {
    rustbox.clear();
    for (i, line) in contents.lines()
                             .skip(position as usize)
                             .enumerate()
                             .take_while(|&(i, _)| i < rustbox.height() - 1) {
        rustbox.print(0, i, rustbox::RB_NORMAL, Color::White, Color::Black, line);
    }

    let info_box = mini_buffer.buffer.iter().map(|c| *c).collect::<String>();
    match mini_buffer.mode {
        MiniBufferMode::Normal => {
            rustbox.print(0,
                          rustbox.height() - 1,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Black,
                          &(":".to_string() + &info_box));
        }
        MiniBufferMode::Info => {
            rustbox.print(0,
                          rustbox.height() - 1,
                          rustbox::RB_NORMAL,
                          Color::Black,
                          Color::White,
                          &info_box);
        }
    }

    rustbox.present();
}

fn word_wrap(width: usize, contents: String) -> String {
    contents.lines()
            .map(|s| s.to_string())
            .flat_map(|s| {
                if s.len() > width {
                    let l = s.chars().take(width).collect::<String>();
                    let r = s.chars().skip(width).collect::<String>();
                    vec![l, r]
                } else {
                    vec![s]
                }
            })
            .map(|s| s + "\n")
            .collect::<String>()
}

#[derive(Eq,PartialEq)]
enum MiniBufferMode {
    Normal,
    Info,
}

struct MiniBuffer {
    buffer: Vec<char>,
    mode: MiniBufferMode,
}

impl MiniBuffer {
    fn new() -> MiniBuffer {
        MiniBuffer {
            buffer: Vec::new(),
            mode: MiniBufferMode::Normal,
        }
    }
    fn clear(&mut self) {
        self.buffer = Vec::new();
        self.mode = MiniBufferMode::Normal;
    }
    fn set_info(&mut self, info: &str) {
        self.buffer = info.chars().collect::<Vec<_>>();
        self.mode = MiniBufferMode::Info;
    }
}
