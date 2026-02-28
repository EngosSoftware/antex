use super::*;
use antex::{Color, ColorMode, StyledText, Text};

#[test]
fn colors_should_work() {
  assert_eq!("\x1b[30mhello", on!().black().s("hello").to_string());
  assert_eq!("\x1b[31mhello", on!().red().s("hello").to_string());
  assert_eq!("\x1b[32mhello", on!().green().s("hello").to_string());
  assert_eq!("\x1b[33mhello", on!().yellow().s("hello").to_string());
  assert_eq!("\x1b[34mhello", on!().blue().s("hello").to_string());
  assert_eq!("\x1b[35mhello", on!().magenta().s("hello").to_string());
  assert_eq!("\x1b[36mhello", on!().cyan().s("hello").to_string());
  assert_eq!("\x1b[37mhello", on!().white().s("hello").to_string());

  assert_eq!("hello", off!().black().s("hello").to_string());
  assert_eq!("hello", off!().red().s("hello").to_string());
  assert_eq!("hello", off!().green().s("hello").to_string());
  assert_eq!("hello", off!().yellow().s("hello").to_string());
  assert_eq!("hello", off!().blue().s("hello").to_string());
  assert_eq!("hello", off!().magenta().s("hello").to_string());
  assert_eq!("hello", off!().cyan().s("hello").to_string());
  assert_eq!("hello", off!().white().s("hello").to_string());
}

#[test]
fn bright_colors_should_work() {
  assert_eq!("\x1b[90mhello", on!().bright_black().s("hello").to_string());
  assert_eq!("\x1b[91mhello", on!().bright_red().s("hello").to_string());
  assert_eq!("\x1b[92mhello", on!().bright_green().s("hello").to_string());
  assert_eq!("\x1b[93mhello", on!().bright_yellow().s("hello").to_string());
  assert_eq!("\x1b[94mhello", on!().bright_blue().s("hello").to_string());
  assert_eq!("\x1b[95mhello", on!().bright_magenta().s("hello").to_string());
  assert_eq!("\x1b[96mhello", on!().bright_cyan().s("hello").to_string());
  assert_eq!("\x1b[97mhello", on!().bright_white().s("hello").to_string());

  assert_eq!("hello", off!().bright_black().s("hello").to_string());
  assert_eq!("hello", off!().bright_red().s("hello").to_string());
  assert_eq!("hello", off!().bright_green().s("hello").to_string());
  assert_eq!("hello", off!().bright_yellow().s("hello").to_string());
  assert_eq!("hello", off!().bright_blue().s("hello").to_string());
  assert_eq!("hello", off!().bright_magenta().s("hello").to_string());
  assert_eq!("hello", off!().bright_cyan().s("hello").to_string());
  assert_eq!("hello", off!().bright_white().s("hello").to_string());
}

#[test]
fn bg_colors_should_work() {
  assert_eq!("\x1b[40mhello", on!().bg_black().s("hello").to_string());
  assert_eq!("\x1b[41mhello", on!().bg_red().s("hello").to_string());
  assert_eq!("\x1b[42mhello", on!().bg_green().s("hello").to_string());
  assert_eq!("\x1b[43mhello", on!().bg_yellow().s("hello").to_string());
  assert_eq!("\x1b[44mhello", on!().bg_blue().s("hello").to_string());
  assert_eq!("\x1b[45mhello", on!().bg_magenta().s("hello").to_string());
  assert_eq!("\x1b[46mhello", on!().bg_cyan().s("hello").to_string());
  assert_eq!("\x1b[47mhello", on!().bg_white().s("hello").to_string());

  assert_eq!("hello", off!().bg_black().s("hello").to_string());
  assert_eq!("hello", off!().bg_red().s("hello").to_string());
  assert_eq!("hello", off!().bg_green().s("hello").to_string());
  assert_eq!("hello", off!().bg_yellow().s("hello").to_string());
  assert_eq!("hello", off!().bg_blue().s("hello").to_string());
  assert_eq!("hello", off!().bg_magenta().s("hello").to_string());
  assert_eq!("hello", off!().bg_cyan().s("hello").to_string());
  assert_eq!("hello", off!().bg_white().s("hello").to_string());
}

#[test]
fn bg_bright_colors_should_work() {
  assert_eq!("\x1b[100mhello", on!().bg_bright_black().s("hello").to_string());
  assert_eq!("\x1b[101mhello", on!().bg_bright_red().s("hello").to_string());
  assert_eq!("\x1b[102mhello", on!().bg_bright_green().s("hello").to_string());
  assert_eq!("\x1b[103mhello", on!().bg_bright_yellow().s("hello").to_string());
  assert_eq!("\x1b[104mhello", on!().bg_bright_blue().s("hello").to_string());
  assert_eq!("\x1b[105mhello", on!().bg_bright_magenta().s("hello").to_string());
  assert_eq!("\x1b[106mhello", on!().bg_bright_cyan().s("hello").to_string());
  assert_eq!("\x1b[107mhello", on!().bg_bright_white().s("hello").to_string());

  assert_eq!("hello", off!().bg_bright_black().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_red().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_green().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_yellow().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_blue().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_magenta().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_cyan().s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_white().s("hello").to_string());
}

#[test]
fn named_colors_should_work() {
  assert_eq!("\x1b[30mhello", on!().color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[31mhello", on!().color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[32mhello", on!().color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[33mhello", on!().color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[34mhello", on!().color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[35mhello", on!().color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[36mhello", on!().color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[37mhello", on!().color(Color::White).s("hello").to_string());
  assert_eq!("\x1b[38;5;132mhello", on!().color(Color::Long(132)).s("hello").to_string());
  assert_eq!("\x1b[38;2;18;34;72mhello", on!().color(Color::Rgb((18, 34, 72))).s("hello").to_string());

  assert_eq!("hello", off!().color(Color::Black).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Red).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Green).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::White).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Long(132)).s("hello").to_string());
  assert_eq!("hello", off!().color(Color::Rgb((18, 34, 72))).s("hello").to_string());
}

#[test]
fn named_bright_colors_should_work() {
  assert_eq!("\x1b[90mhello", on!().bright_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[91mhello", on!().bright_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[92mhello", on!().bright_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[93mhello", on!().bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[94mhello", on!().bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[95mhello", on!().bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[96mhello", on!().bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[97mhello", on!().bright_color(Color::White).s("hello").to_string());

  assert_eq!("hello", off!().bright_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", off!().bright_color(Color::White).s("hello").to_string());
}

#[test]
fn named_bg_colors_should_work() {
  assert_eq!("\x1b[40mhello", on!().bg_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[41mhello", on!().bg_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[42mhello", on!().bg_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[43mhello", on!().bg_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[44mhello", on!().bg_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[45mhello", on!().bg_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[46mhello", on!().bg_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[47mhello", on!().bg_color(Color::White).s("hello").to_string());
  assert_eq!("\x1b[48;5;132mhello", on!().bg_color(Color::Long(132)).s("hello").to_string());
  assert_eq!("\x1b[48;2;18;34;72mhello", on!().bg_color(Color::Rgb((18, 34, 72))).s("hello").to_string());

  assert_eq!("hello", off!().bg_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::White).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Long(132)).s("hello").to_string());
  assert_eq!("hello", off!().bg_color(Color::Rgb((18, 34, 72))).s("hello").to_string());
}

#[test]
fn named_bg_bright_colors_should_work() {
  assert_eq!("\x1b[100mhello", on!().bg_bright_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[101mhello", on!().bg_bright_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[102mhello", on!().bg_bright_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[103mhello", on!().bg_bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[104mhello", on!().bg_bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[105mhello", on!().bg_bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[106mhello", on!().bg_bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[107mhello", on!().bg_bright_color(Color::White).s("hello").to_string());

  assert_eq!("hello", off!().bg_bright_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color(Color::White).s("hello").to_string());
}

#[test]
fn color_8_should_work() {
  assert_eq!("\x1b[30mhello", on!().color_8(0).s("hello").to_string());
  assert_eq!("\x1b[31mhello", on!().color_8(1).s("hello").to_string());
  assert_eq!("\x1b[32mhello", on!().color_8(2).s("hello").to_string());
  assert_eq!("\x1b[33mhello", on!().color_8(3).s("hello").to_string());
  assert_eq!("\x1b[34mhello", on!().color_8(4).s("hello").to_string());
  assert_eq!("\x1b[35mhello", on!().color_8(5).s("hello").to_string());
  assert_eq!("\x1b[36mhello", on!().color_8(6).s("hello").to_string());
  assert_eq!("\x1b[37mhello", on!().color_8(7).s("hello").to_string());
  assert_eq!("hello", on!().color_8(8).s("hello").to_string());

  assert_eq!("hello", off!().color_8(0).s("hello").to_string());
  assert_eq!("hello", off!().color_8(1).s("hello").to_string());
  assert_eq!("hello", off!().color_8(2).s("hello").to_string());
  assert_eq!("hello", off!().color_8(3).s("hello").to_string());
  assert_eq!("hello", off!().color_8(4).s("hello").to_string());
  assert_eq!("hello", off!().color_8(5).s("hello").to_string());
  assert_eq!("hello", off!().color_8(6).s("hello").to_string());
  assert_eq!("hello", off!().color_8(7).s("hello").to_string());
  assert_eq!("hello", off!().color_8(8).s("hello").to_string());
}

#[test]
fn bright_color_8_should_work() {
  assert_eq!("\x1b[91mhello", on!().bright_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[92mhello", on!().bright_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[90mhello", on!().bright_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[93mhello", on!().bright_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[94mhello", on!().bright_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[95mhello", on!().bright_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[96mhello", on!().bright_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[97mhello", on!().bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", on!().bright_color_8(8).s("hello").to_string());

  assert_eq!("hello", off!().bright_color_8(0).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(1).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(2).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(3).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(4).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(5).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(6).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", off!().bright_color_8(8).s("hello").to_string());
}

#[test]
fn bg_color_8_should_work() {
  assert_eq!("\x1b[40mhello", on!().bg_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[41mhello", on!().bg_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[42mhello", on!().bg_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[43mhello", on!().bg_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[44mhello", on!().bg_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[45mhello", on!().bg_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[46mhello", on!().bg_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[47mhello", on!().bg_color_8(7).s("hello").to_string());
  assert_eq!("hello", on!().bg_color_8(8).s("hello").to_string());

  assert_eq!("hello", off!().bg_color_8(0).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(1).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(2).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(3).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(4).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(5).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(6).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(7).s("hello").to_string());
  assert_eq!("hello", off!().bg_color_8(8).s("hello").to_string());
}

#[test]
fn bg_bright_color_8_should_work() {
  assert_eq!("\x1b[100mhello", on!().bg_bright_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[101mhello", on!().bg_bright_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[102mhello", on!().bg_bright_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[103mhello", on!().bg_bright_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[104mhello", on!().bg_bright_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[105mhello", on!().bg_bright_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[106mhello", on!().bg_bright_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[107mhello", on!().bg_bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", on!().bg_bright_color_8(8).s("hello").to_string());

  assert_eq!("hello", off!().bg_bright_color_8(0).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(1).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(2).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(3).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(4).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(5).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(6).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", off!().bg_bright_color_8(8).s("hello").to_string());
}

#[test]
fn color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\x1b[38;5;{}mhello", i), on!().color_256(i).s("hello").to_string());
    assert_eq!("hello", off!().color_256(i).s("hello").to_string());
  }
}

#[test]
fn bg_color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\x1b[48;5;{}mhello", i), on!().bg_color_256(i).s("hello").to_string());
    assert_eq!("hello", off!().bg_color_256(i).s("hello").to_string());
  }
}

#[test]
fn color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\x1b[38;2;{};{};{}mhello", r, g, b), on!().color_rgb((r, g, b)).s("hello").to_string());
        assert_eq!("hello", off!().color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}

#[test]
fn bg_color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\x1b[48;2;{};{};{}mhello", r, g, b), on!().bg_color_rgb((r, g, b)).s("hello").to_string());
        assert_eq!("hello", off!().bg_color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}

#[test]
fn color_conversion_should_work() {
  assert_eq!("\x1b[30mhello", on!().color(0.into()).s("hello").to_string());
  assert_eq!("\x1b[31mhello", on!().color(1.into()).s("hello").to_string());
  assert_eq!("\x1b[32mhello", on!().color(2.into()).s("hello").to_string());
  assert_eq!("\x1b[33mhello", on!().color(3.into()).s("hello").to_string());
  assert_eq!("\x1b[34mhello", on!().color(4.into()).s("hello").to_string());
  assert_eq!("\x1b[35mhello", on!().color(5.into()).s("hello").to_string());
  assert_eq!("\x1b[36mhello", on!().color(6.into()).s("hello").to_string());
  assert_eq!("\x1b[37mhello", on!().color(7.into()).s("hello").to_string());
  assert_eq!("\x1b[38;5;132mhello", on!().color(132.into()).s("hello").to_string());
  assert_eq!("\x1b[38;2;10;73;134mhello", on!().color((10, 73, 134).into()).s("hello").to_string());
}
