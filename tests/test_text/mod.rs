use super::*;
use antex::{ColorMode, StyledText, Text, always, auto, never};

mod add;
mod characters;
mod colors;
mod format;
mod formatting;

#[test]
fn text_default_should_work() {
  let text = Text::default().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello", chars: 5 }}"#, cm()), format!("{:?}", text));
}

#[test]
fn text_auto_should_work() {
  let text = auto().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello", chars: 5 }}"#, cm()), format!("{:?}", text));
}

#[test]
fn text_from_cm_default_should_work() {
  let mut text: Text = ColorMode::default().into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "Hello", chars: 5 }}"#, cm()), format!("{:?}", text));
  assert_eq!(format!(r#"Text {{ cm: {}, content: "", chars: 0 }}"#, cm()), format!("{:?}", Text::auto()));
}

#[test]
fn text_off_should_work() {
  let text = Text::new(ColorMode::Off).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: Off, content: "", chars: 0 }"#, format!("{:?}", Text::off()));
}

#[test]
fn text_never_should_work() {
  let text = never().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: Off, content: "", chars: 0 }"#, format!("{:?}", Text::off()));
}

#[test]
fn text_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: Off, content: "", chars: 0 }"#, format!("{:?}", Text::off()));
}

#[test]
fn text_on_should_work() {
  let text = Text::new(ColorMode::On).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: On, content: "", chars: 0 }"#, format!("{:?}", Text::on()));
}

#[test]
fn text_always_should_work() {
  let text = always().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: On, content: "", chars: 0 }"#, format!("{:?}", Text::on()));
}

#[test]
fn text_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "Hello", chars: 5 }"#, format!("{:?}", text));
  assert_eq!(r#"Text { cm: On, content: "", chars: 0 }"#, format!("{:?}", Text::on()));
}

#[test]
fn text_coloured_off_should_work() {
  let text = Text::new(ColorMode::Off).yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello", chars: 5 }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { cm: Off, content: "Hello", chars: 5 }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_on_should_work() {
  let text = Text::new(ColorMode::On).yellow().s("Hello");
  assert_eq!("\x1b[33mHello", text.to_string());
  assert_eq!("\x1b[33mHello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello", chars: 5 }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.yellow().s("Hello");
  assert_eq!("\x1b[33mHello", text.to_string());
  assert_eq!("\x1b[33mHello", format!("{}", text));
  assert_eq!(r#"Text { cm: On, content: "\u{1b}[33mHello", chars: 5 }"#, format!("{:?}", text));
}
