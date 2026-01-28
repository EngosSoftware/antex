use antex::{Color, ColorMode, StyledText, Text};

macro_rules! t {
  () => {
    Text::new(ColorMode::On)
  };
}

#[test]
fn colors_should_work() {
  assert_eq!("\u{1b}[30mhello", t!().black().s("hello").to_string());
  assert_eq!("\u{1b}[31mhello", t!().red().s("hello").to_string());
  assert_eq!("\u{1b}[32mhello", t!().green().s("hello").to_string());
  assert_eq!("\u{1b}[33mhello", t!().yellow().s("hello").to_string());
  assert_eq!("\u{1b}[34mhello", t!().blue().s("hello").to_string());
  assert_eq!("\u{1b}[35mhello", t!().magenta().s("hello").to_string());
  assert_eq!("\u{1b}[36mhello", t!().cyan().s("hello").to_string());
  assert_eq!("\u{1b}[37mhello", t!().white().s("hello").to_string());
}

#[test]
fn bg_colors_should_work() {
  assert_eq!("\u{1b}[40mhello", t!().bg_black().s("hello").to_string());
  assert_eq!("\u{1b}[41mhello", t!().bg_red().s("hello").to_string());
  assert_eq!("\u{1b}[42mhello", t!().bg_green().s("hello").to_string());
  assert_eq!("\u{1b}[43mhello", t!().bg_yellow().s("hello").to_string());
  assert_eq!("\u{1b}[44mhello", t!().bg_blue().s("hello").to_string());
  assert_eq!("\u{1b}[45mhello", t!().bg_magenta().s("hello").to_string());
  assert_eq!("\u{1b}[46mhello", t!().bg_cyan().s("hello").to_string());
  assert_eq!("\u{1b}[47mhello", t!().bg_white().s("hello").to_string());
}

#[test]
fn named_colors_should_work() {
  assert_eq!("\u{1b}[30mhello", t!().color(Color::Black).s("hello").to_string());
  assert_eq!("\u{1b}[31mhello", t!().color(Color::Red).s("hello").to_string());
  assert_eq!("\u{1b}[32mhello", t!().color(Color::Green).s("hello").to_string());
  assert_eq!("\u{1b}[33mhello", t!().color(Color::Yellow).s("hello").to_string());
  assert_eq!("\u{1b}[34mhello", t!().color(Color::Blue).s("hello").to_string());
  assert_eq!("\u{1b}[35mhello", t!().color(Color::Magenta).s("hello").to_string());
  assert_eq!("\u{1b}[36mhello", t!().color(Color::Cyan).s("hello").to_string());
  assert_eq!("\u{1b}[37mhello", t!().color(Color::White).s("hello").to_string());
}

#[test]
fn named_bg_colors_should_work() {
  assert_eq!("\u{1b}[40mhello", t!().bg_color(Color::Black).s("hello").to_string());
  assert_eq!("\u{1b}[41mhello", t!().bg_color(Color::Red).s("hello").to_string());
  assert_eq!("\u{1b}[42mhello", t!().bg_color(Color::Green).s("hello").to_string());
  assert_eq!("\u{1b}[43mhello", t!().bg_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\u{1b}[44mhello", t!().bg_color(Color::Blue).s("hello").to_string());
  assert_eq!("\u{1b}[45mhello", t!().bg_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\u{1b}[46mhello", t!().bg_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\u{1b}[47mhello", t!().bg_color(Color::White).s("hello").to_string());
}

#[test]
fn color_8_should_work() {
  assert_eq!("\u{1b}[30mhello", t!().color_8(0).s("hello").to_string());
  assert_eq!("\u{1b}[31mhello", t!().color_8(1).s("hello").to_string());
  assert_eq!("\u{1b}[32mhello", t!().color_8(2).s("hello").to_string());
  assert_eq!("\u{1b}[33mhello", t!().color_8(3).s("hello").to_string());
  assert_eq!("\u{1b}[34mhello", t!().color_8(4).s("hello").to_string());
  assert_eq!("\u{1b}[35mhello", t!().color_8(5).s("hello").to_string());
  assert_eq!("\u{1b}[36mhello", t!().color_8(6).s("hello").to_string());
  assert_eq!("\u{1b}[37mhello", t!().color_8(7).s("hello").to_string());
}

#[test]
fn bg_color_8_should_work() {
  assert_eq!("\u{1b}[40mhello", t!().bg_color_8(0).s("hello").to_string());
  assert_eq!("\u{1b}[41mhello", t!().bg_color_8(1).s("hello").to_string());
  assert_eq!("\u{1b}[42mhello", t!().bg_color_8(2).s("hello").to_string());
  assert_eq!("\u{1b}[43mhello", t!().bg_color_8(3).s("hello").to_string());
  assert_eq!("\u{1b}[44mhello", t!().bg_color_8(4).s("hello").to_string());
  assert_eq!("\u{1b}[45mhello", t!().bg_color_8(5).s("hello").to_string());
  assert_eq!("\u{1b}[46mhello", t!().bg_color_8(6).s("hello").to_string());
  assert_eq!("\u{1b}[47mhello", t!().bg_color_8(7).s("hello").to_string());
}

#[test]
fn color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\u{1b}[38;5;{}mhello", i), t!().color_256(i).s("hello").to_string());
  }
}

#[test]
fn bg_color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\u{1b}[48;5;{}mhello", i), t!().bg_color_256(i).s("hello").to_string());
  }
}

#[test]
fn color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\u{1b}[38;2;{};{};{}mhello", r, g, b), t!().color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}

#[test]
fn bg_color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\u{1b}[48;2;{};{};{}mhello", r, g, b), t!().bg_color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}
