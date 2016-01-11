

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

    let expected = Buffer{
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


    let expected = Buffer{
        vec: vec![
            Line{line_num: 1, line: "1234567890".to_string()},
            Line{line_num: 1, line: "123456789".to_string()},
            Line{line_num: 2, line: "abcdefgh".to_string()},
            ],
        len: 3,
    };
    assert_eq!(test, expected);
}
