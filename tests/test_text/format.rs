use super::*;
use antex::{StyledText, Text};

fn text() -> Text {
  always().blue().s("Hello").reset().s(" ☺ ").yellow().s("world").s(" ").magenta().s(1.999).reset()
}

fn text_plural(n: usize) -> Text {
  always().blue().plural("☺Hello", n).reset()
}

fn text_repeat() -> Text {
  always().blue().repeat("☺", 3).reset()
}

#[test]
fn width_should_work() {
  assert_eq!("\x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m           ", format!("{:30}", text()));
}

#[test]
fn align_left_should_work() {
  assert_eq!("\x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m           ", format!("{:<30}", text()));
}

#[test]
fn align_right_should_work() {
  assert_eq!("           \x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m", format!("{:>30}", text()));
}

#[test]
fn align_center_should_work() {
  assert_eq!("     \x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m      ", format!("{:^30}", text()));
}

#[test]
fn align_left_fill_should_work() {
  assert_eq!("\x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m-----------", format!("{:-<30}", text()));
}

#[test]
fn align_right_fill_should_work() {
  assert_eq!("-----------\x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m", format!("{:->30}", text()));
}

#[test]
fn align_center_fill_should_work() {
  assert_eq!("-----\x1b[34mHello\x1b[0m ☺ \x1b[33mworld \x1b[35m1.999\x1b[0m------", format!("{:-^30}", text()));
}

#[test]
fn width_plural_should_work() {
  assert_eq!("\x1b[34m☺Hello\x1b[0m                        ", format!("{:30}", text_plural(1)));
  assert_eq!("\x1b[34m☺Hellos\x1b[0m                       ", format!("{:30}", text_plural(2)));
}

#[test]
fn align_left_plural_should_work() {
  assert_eq!("\x1b[34m☺Hello\x1b[0m                        ", format!("{:<30}", text_plural(1)));
  assert_eq!("\x1b[34m☺Hellos\x1b[0m                       ", format!("{:<30}", text_plural(2)));
}

#[test]
fn align_right_plural_should_work() {
  assert_eq!("                        \x1b[34m☺Hello\x1b[0m", format!("{:>30}", text_plural(1)));
  assert_eq!("                       \x1b[34m☺Hellos\x1b[0m", format!("{:>30}", text_plural(2)));
}

#[test]
fn align_center_plural_should_work() {
  assert_eq!("            \x1b[34m☺Hello\x1b[0m            ", format!("{:^30}", text_plural(1)));
  assert_eq!("           \x1b[34m☺Hellos\x1b[0m            ", format!("{:^30}", text_plural(2)));
}

#[test]
fn align_left_fill_plural_should_work() {
  assert_eq!("\x1b[34m☺Hello\x1b[0m------------------------", format!("{:-<30}", text_plural(1)));
  assert_eq!("\x1b[34m☺Hellos\x1b[0m-----------------------", format!("{:-<30}", text_plural(2)));
}

#[test]
fn align_right_fill_plural_should_work() {
  assert_eq!("------------------------\x1b[34m☺Hello\x1b[0m", format!("{:->30}", text_plural(1)));
  assert_eq!("-----------------------\x1b[34m☺Hellos\x1b[0m", format!("{:->30}", text_plural(2)));
}

#[test]
fn align_center_fill_plural_should_work() {
  assert_eq!("------------\x1b[34m☺Hello\x1b[0m------------", format!("{:-^30}", text_plural(1)));
  assert_eq!("-----------\x1b[34m☺Hellos\x1b[0m------------", format!("{:-^30}", text_plural(2)));
}

#[test]
fn width_repeat_should_work() {
  assert_eq!("\x1b[34m☺☺☺\x1b[0m                           ", format!("{:30}", text_repeat()));
}

#[test]
fn align_left_repeat_should_work() {
  assert_eq!("\x1b[34m☺☺☺\x1b[0m                           ", format!("{:<30}", text_repeat()));
}

#[test]
fn align_right_repeat_should_work() {
  assert_eq!("                           \x1b[34m☺☺☺\x1b[0m", format!("{:>30}", text_repeat()));
}

#[test]
fn align_center_repeat_should_work() {
  assert_eq!("             \x1b[34m☺☺☺\x1b[0m              ", format!("{:^30}", text_repeat()));
}

#[test]
fn align_left_fill_repeat_should_work() {
  assert_eq!("\x1b[34m☺☺☺\x1b[0m---------------------------", format!("{:-<30}", text_repeat()));
}

#[test]
fn align_right_fill_repeat_should_work() {
  assert_eq!("---------------------------\x1b[34m☺☺☺\x1b[0m", format!("{:->30}", text_repeat()));
}

#[test]
fn align_center_fill_repeat_should_work() {
  assert_eq!("-------------\x1b[34m☺☺☺\x1b[0m--------------", format!("{:-^30}", text_repeat()));
}
