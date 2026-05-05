use antex::{StyledText, never};

#[test]
fn content_fill_should_work() {
  assert_eq!("☺hello   ", format!("{}", never().s("☺hello").fill(' ', 9)));
}

#[test]
fn content_fill_short_should_work() {
  assert_eq!("☺hello", format!("{}", never().s("☺hello").fill(' ', 3)));
}

#[test]
fn content_fill_custom_should_work() {
  assert_eq!("☺hello☺☺☺☺", format!("{}", never().s("☺hello").fill('☺', 10)));
}
