use antex::{StyledText, never};

#[test]
fn content_pad_should_work() {
  assert_eq!("---☺hello", format!("{}", never().pad('-', 3, "☺hello")));
}

#[test]
fn content_indent_should_work() {
  assert_eq!("   ☺hello", format!("{}", never().indent(3, "☺hello")));
}

#[test]
fn content_pad_left_should_work() {
  assert_eq!("he", format!("{}", never().pad_left('-', "hello", 2)));
  assert_eq!("hello", format!("{}", never().pad_left('-', "hello", 5)));
  assert_eq!("-hello", format!("{}", never().pad_left('-', "hello", 6)));
  assert_eq!("----☺hello", format!("{}", never().pad_left('-', "☺hello", 10)));
}

#[test]
fn content_pad_right_should_work() {
  assert_eq!("he", format!("{}", never().pad_right('-', "hello", 2)));
  assert_eq!("hello", format!("{}", never().pad_right('-', "hello", 5)));
  assert_eq!("hello-", format!("{}", never().pad_right('-', "hello", 6)));
  assert_eq!("☺hello----", format!("{}", never().pad_right('-', "☺hello", 10)));
}

#[test]
fn content_pad_center_should_work() {
  assert_eq!("he", format!("{}", never().pad_center('-', "hello", 2)));
  assert_eq!("hello", format!("{}", never().pad_center('-', "hello", 5)));
  assert_eq!("hello-", format!("{}", never().pad_center('-', "hello", 6)));
  assert_eq!("-hello-", format!("{}", never().pad_center('-', "hello", 7)));
  assert_eq!("--☺hello--", format!("{}", never().pad_center('-', "☺hello", 10)));
}
