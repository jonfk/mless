
use std::env;
use std::error::Error;
use std::default::Default;
use std::io::prelude::*;
use std::fs::File;

use super::word_wrap;

// #[test]
// fn word_wrap() {

//     let contents = r#"1234567890123456789"#;

//     let width = 10;

//     let test = contents.lines().map(|s| s.to_string() ).flat_map(|s| {
//         if s.len() > width {
//             let mut l = s.chars().take(width).collect::<String>();
//             let mut r = s.chars().skip(width).collect::<String>();
//             vec![l, r]
//         } else {
//             vec![s]
//         }
//     }).collect::<Vec<_>>();

//     let expected = vec!["1234567890".to_string(), "123456789".to_string()];
//     assert_eq!(test,expected);
// }


#[test]
fn test_word_wrap() {
    let contents = "1234567890123456789";
    let test = word_wrap(10, contents.to_string());

    let expected = "1234567890\n123456789\n";
    assert_eq!(test, expected);
}
