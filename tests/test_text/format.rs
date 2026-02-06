use super::*;
use antex::{StyledText, Text};

fn text() -> Text {
  always().blue().s("Hello").normal().s(" ").yellow().s("world").s(" ").magenta().s(1.999).normal()
}

#[test]
fn width_should_work() {
  assert_eq!("\u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m             ", format!("{:30}", text()));
}

#[test]
fn align_left_should_work() {
  assert_eq!("\u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m             ", format!("{:<30}", text()));
}

#[test]
fn align_right_should_work() {
  assert_eq!("             \u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m", format!("{:>30}", text()));
}

#[test]
fn align_center_should_work() {
  assert_eq!("      \u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m       ", format!("{:^30}", text()));
}

#[test]
fn align_left_fill_should_work() {
  assert_eq!("\u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m-------------", format!("{:-<30}", text()));
}

#[test]
fn align_right_fill_should_work() {
  assert_eq!("-------------\u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m", format!("{:->30}", text()));
}

#[test]
fn align_center_fill_should_work() {
  assert_eq!("------\u{1b}[34mHello\u{1b}[0m \u{1b}[33mworld \u{1b}[35m1.999\u{1b}[0m-------", format!("{:-^30}", text()));
}
