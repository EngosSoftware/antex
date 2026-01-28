mod test_text_add;

use antex::{ColorMode, StyledText, Text};

#[test]
fn text_default_should_work() {
  let text = Text::default().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_from_cm_default_should_work() {
  let mut text: Text = ColorMode::default().into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_off_should_work() {
  let text = Text::new(ColorMode::Off).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_on_should_work() {
  let text = Text::new(ColorMode::On).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_off_should_work() {
  let text = Text::new(ColorMode::Off).yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: Off, content: "Hello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_on_should_work() {
  let text = Text::new(ColorMode::On).yellow().s("Hello");
  assert_eq!("\u{1b}[33mHello", text.to_string());
  assert_eq!("\u{1b}[33mHello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "\u{1b}[33mHello" }"#, format!("{:?}", text));
}

#[test]
fn text_coloured_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.yellow().s("Hello");
  assert_eq!("\u{1b}[33mHello", text.to_string());
  assert_eq!("\u{1b}[33mHello", format!("{}", text));
  assert_eq!(r#"Text { color_mode: On, content: "\u{1b}[33mHello" }"#, format!("{:?}", text));
}
