use super::*;

mod test_add;
mod test_characters;
mod test_colors;
mod test_formatting;

use antex::{ColorMode, StyledText, Text};

#[test]
fn text_default_should_work() {
  let text = Text::default().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello" }}"#, cm()), format!("{:?}", text));
}

#[test]
fn text_from_cm_default_should_work() {
  let mut text: Text = ColorMode::default().into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello" }}"#, cm()), format!("{:?}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "" }}"#, cm()), format!("{:?}", Text::auto()));
}

#[test]
fn text_off_should_work() {
  let text = Text::new(ColorMode::Off).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello" }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: Off, content: "" }"#, format!("{:?}", Text::off()));
}

#[test]
fn text_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello" }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: Off, content: "" }"#, format!("{:?}", Text::off()));
}

#[test]
fn text_on_should_work() {
  let text = Text::new(ColorMode::On).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello" }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: On, content: "" }"#, format!("{:?}", Text::on()));
}

#[test]
fn text_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello" }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: On, content: "" }"#, format!("{:?}", Text::on()));
}

#[test]
fn text_coloured_off_should_work() {
  let text = Text::new(ColorMode::Off).yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_on_should_work() {
  let text = Text::new(ColorMode::On).yellow().s("Hello");
  assert_eq!("\u{1b}[33mHello", text.to_string());
  assert_eq!("\u{1b}[33mHello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.yellow().s("Hello");
  assert_eq!("\u{1b}[33mHello", text.to_string());
  assert_eq!("\u{1b}[33mHello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello" }"#, format!("{:?}", text));
}
