use super::*;
use antex::StyledText;

#[test]
fn bold_should_work() {
  assert_eq!("\x1b[1mhello", always().bold().s("hello").to_string());
  assert_eq!("hello", never().bold().s("hello").to_string());
}

#[test]
fn italic_should_work() {
  assert_eq!("\x1b[3mhello", always().italic().s("hello").to_string());
  assert_eq!("hello", never().italic().s("hello").to_string());
}

#[test]
fn underline_should_work() {
  assert_eq!("\x1b[4mhello", always().underline().s("hello").to_string());
  assert_eq!("hello", never().underline().s("hello").to_string());
}
