

#[test]
fn test_word_wrap() {

    use super::word_wrap;

    let contents = "1234567890123456789";
    let test = word_wrap(contents.to_string(), 10);

    let expected = "1234567890\n123456789\n";
    assert_eq!(test, expected);
}

#[test]
fn test_to_buffer() {
    use super::Buffer;
    use super::Line;
    let contents = "1234567890123456789\nabcdefgh";

    let test = Buffer::new(contents.to_string());

    let expected = Buffer {
        vec: vec![
            Line{line_num: 1, line: "1234567890123456789".to_string()},
            Line{line_num: 2, line: "abcdefgh".to_string()},
            ],
        len: 2,
    };
    assert_eq!(test, expected);
}

#[test]
fn test_line_wrap_buffer() {
    use super::Buffer;
    use super::Line;
    let contents = "1234567890123456789\nabcdefgh";

    let test = Buffer::new(contents.to_string()).line_wrap(10);


    let expected = Buffer {
        vec: vec![
            Line{line_num: 1, line: "1234567890".to_string()},
            Line{line_num: 1, line: "123456789".to_string()},
            Line{line_num: 2, line: "abcdefgh".to_string()},
            ],
        len: 3,
    };
    assert_eq!(test, expected);
}

#[test]
#[ignore]
/// Creates a large file of about 1G
///
fn create_large_file() {
    use std::io::prelude::*;
    use std::fs::File;
    use std::fs;

    let filename = "large_file.txt";

    let mut f = File::create(filename).unwrap();

    let mut meta = fs::metadata(filename).unwrap();
    let mut i: usize = 0;

    while meta.len() < 1000000000 {
        f.write_all( &format!("{} {}", i, "Hello, world!\n").into_bytes()[..]).unwrap();
        meta = fs::metadata(filename).unwrap();
        i += 1;
    }
}
