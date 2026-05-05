use super::*;
use antex::{ColorMode, StyledText, Text};

#[test]
fn text_add_default_should_work() {
  let a = Text::default().s("Hello");
  let b = Text::default().s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
}

#[test]
fn text_add_off_should_work() {
  let a = Text::new(ColorMode::Off).s("Hello");
  let b = Text::new(ColorMode::Off).s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
}

#[test]
fn text_add_on_should_work() {
  let a = Text::new(ColorMode::On).s("Hello");
  let b = Text::new(ColorMode::On).s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
}

#[test]
fn text_add_on_coloured_should_work() {
  let a = Text::new(ColorMode::On).yellow().s("Hello");
  let b = Text::new(ColorMode::On).green().s(" world!");
  let text = a + b;
  assert_eq!("\x1b[33mHello\x1b[32m world!", text.to_string());
  assert_eq!("\x1b[33mHello\x1b[32m world!", format!("{}", text));
}

#[test]
fn text_add_on_coloured_clear_should_work() {
  let a = Text::new(ColorMode::On).yellow().s("Hello").reset();
  let b = Text::new(ColorMode::On).s(" world!");
  let text = a + b;
  assert_eq!("\x1b[33mHello\x1b[0m world!", text.to_string());
  assert_eq!("\x1b[33mHello\x1b[0m world!", format!("{}", text));
}

#[test]
fn adding_multiple_types_should_work() {
  let a = never().s("hello");
  let b = never().s("world");
  let c = '!';
  let d = "!".to_string();
  let e = "☺";

  let greeting = a.clone() + b.clone();
  assert_eq!("helloworld", greeting.to_string());

  let greeting = a.clone() + &b;
  assert_eq!("helloworld", greeting.to_string());

  let greeting = a.clone() + c;
  assert_eq!("hello!", greeting.to_string());

  let greeting = a.clone() + d.as_str();
  assert_eq!("hello!", greeting.to_string());

  let greeting = a.clone() + " " + b.clone() + c;
  assert_eq!("hello world!", greeting.to_string());

  let greeting = a.clone() + " " + &b + d.clone();
  assert_eq!("hello world!", greeting.to_string());

  let greeting = c + &a;
  assert_eq!("!hello", greeting.to_string());

  let greeting = d.clone() + &a;
  assert_eq!("!hello", greeting.to_string());

  let greeting = c + a.clone();
  assert_eq!("!hello", greeting.to_string());

  let greeting = &d + a.clone();
  assert_eq!("!hello", greeting.to_string());

  let greeting = d.clone() + a.clone();
  assert_eq!("!hello", greeting.to_string());

  let greeting = a.clone() + &d;
  assert_eq!("hello!", greeting.to_string());

  let greeting = e + a.clone();
  assert_eq!("☺hello", greeting.to_string());

  let greeting = a + e;
  assert_eq!("hello☺", greeting.to_string());
}
