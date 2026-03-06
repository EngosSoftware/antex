use antex::{StyledText, never};

#[test]
fn content_align_left_should_work() {
  assert_eq!("he", format!("{}", never().align_left("hello", 2)));
  assert_eq!("hello", format!("{}", never().align_left("hello", 5)));
  assert_eq!("hello ", format!("{}", never().align_left("hello", 6)));
  assert_eq!("☺hello    ", format!("{}", never().align_left("☺hello", 10)));
}

#[test]
fn content_align_right_should_work() {
  assert_eq!("he", format!("{}", never().align_right("hello", 2)));
  assert_eq!("hello", format!("{}", never().align_right("hello", 5)));
  assert_eq!(" hello", format!("{}", never().align_right("hello", 6)));
  assert_eq!("    ☺hello", format!("{}", never().align_right("☺hello", 10)));
}

#[test]
fn content_align_center_should_work() {
  assert_eq!("he", format!("{}", never().align_center("hello", 2)));
  assert_eq!("hello", format!("{}", never().align_center("hello", 5)));
  assert_eq!("hello ", format!("{}", never().align_center("hello", 6)));
  assert_eq!(" hello ", format!("{}", never().align_center("hello", 7)));
  assert_eq!("  ☺hello  ", format!("{}", never().align_center("☺hello", 10)));
}
