use super::*;
use antex::{ColorMode, StyledText, Text};

#[test]
fn bold_should_work() {
  assert_eq!("\u{1b}[1mhello", on!().bold().s("hello").to_string());
  assert_eq!("hello", off!().bold().s("hello").to_string());
}

#[test]
fn italic_should_work() {
  assert_eq!("\u{1b}[3mhello", on!().italic().s("hello").to_string());
  assert_eq!("hello", off!().italic().s("hello").to_string());
}

#[test]
fn underline_should_work() {
  assert_eq!("\u{1b}[4mhello", on!().underline().s("hello").to_string());
  assert_eq!("hello", off!().underline().s("hello").to_string());
}
