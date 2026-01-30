use super::*;
use antex::{ColorMode, StyledText, Text};

#[test]
fn text_add_default_should_work() {
  let a = Text::default().s("Hello");
  let b = Text::default().s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello world!" }}"#, cm()), format!("{:?}", text));
}

#[test]
fn text_add_off_should_work() {
  let a = Text::new(ColorMode::Off).s("Hello");
  let b = Text::new(ColorMode::Off).s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello world!" }"#, format!("{:?}", text));
}

#[test]
fn text_add_on_should_work() {
  let a = Text::new(ColorMode::On).s("Hello");
  let b = Text::new(ColorMode::On).s(" world!");
  let text = a + b;
  assert_eq!("Hello world!", text.to_string());
  assert_eq!("Hello world!", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello world!" }"#, format!("{:?}", text));
}

#[test]
fn text_add_on_coloured_should_work() {
  let a = Text::new(ColorMode::On).yellow().s("Hello");
  let b = Text::new(ColorMode::On).green().s(" world!");
  let text = a + b;
  assert_eq!("\u{1b}[33mHello\u{1b}[32m world!", text.to_string());
  assert_eq!("\u{1b}[33mHello\u{1b}[32m world!", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello\u{1b}[32m world!" }"#, format!("{:?}", text));
}

#[test]
fn text_add_on_coloured_clear_should_work() {
  let a = Text::new(ColorMode::On).yellow().s("Hello").clear();
  let b = Text::new(ColorMode::On).s(" world!");
  let text = a + b;
  assert_eq!("\u{1b}[33mHello\u{1b}[0m world!", text.to_string());
  assert_eq!("\u{1b}[33mHello\u{1b}[0m world!", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello\u{1b}[0m world!" }"#, format!("{:?}", text));
}
