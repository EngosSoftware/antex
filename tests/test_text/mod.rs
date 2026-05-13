use antex::{ColorMode, StyledText, Text, always, auto, never};

mod add;
mod align;
mod characters;
mod choose;
mod colors;
mod filling;
mod format;
mod formatting;
mod matches;
mod padding;
mod plural;
mod writer;

#[test]
fn text_default_should_work() {
  let text = Text::default().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_auto_should_work() {
  let text = auto().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_from_cm_default_should_work() {
  let mut text: Text = ColorMode::default().into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_off_should_work() {
  let text = Text::new(ColorMode::Off).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_never_should_work() {
  let text = never().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_on_should_work() {
  let text = Text::new(ColorMode::On).s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_always_should_work() {
  let text = always().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_coloured_off_should_work() {
  let text = Text::new(ColorMode::Off).yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_coloured_from_cm_off_should_work() {
  let mut text: Text = ColorMode::Off.into();
  text = text.yellow().s("Hello");
  assert_eq!("Hello", text.to_string());
  assert_eq!("Hello", format!("{}", text));
}

#[test]
fn text_coloured_on_should_work() {
  let text = Text::new(ColorMode::On).yellow().s("Hello");
  assert_eq!("\x1b[33mHello", text.to_string());
  assert_eq!("\x1b[33mHello", format!("{}", text));
}

#[test]
fn text_coloured_from_cm_on_should_work() {
  let mut text: Text = ColorMode::On.into();
  text = text.yellow().s("Hello");
  assert_eq!("\x1b[33mHello", text.to_string());
  assert_eq!("\x1b[33mHello", format!("{}", text));
}

#[test]
fn text_as_ref_should_work() {
  let t1 = Text::default().yellow().s("Hello").reset();
  let t2 = Text::default().red().s("Hello").reset();

  fn a(t: impl AsRef<Text>) -> String {
    t.as_ref().characters()
  }

  fn b(t: impl AsRef<Text>) -> String {
    t.as_ref().characters()
  }

  assert_eq!(a(t1), b(t2));
}
