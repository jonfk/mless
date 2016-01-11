

#[test]
fn test_word_wrap() {

    use super::word_wrap;

    let contents = "1234567890123456789";
    let test = word_wrap(10, contents.to_string());

    let expected = "1234567890\n123456789\n";
    assert_eq!(test, expected);
}
