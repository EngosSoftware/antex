use super::*;
use antex::{Color, StyledText};

#[test]
fn colors_should_work() {
  assert_eq!("\x1b[30mhello", always().black().s("hello").to_string());
  assert_eq!("\x1b[31mhello", always().red().s("hello").to_string());
  assert_eq!("\x1b[32mhello", always().green().s("hello").to_string());
  assert_eq!("\x1b[33mhello", always().yellow().s("hello").to_string());
  assert_eq!("\x1b[34mhello", always().blue().s("hello").to_string());
  assert_eq!("\x1b[35mhello", always().magenta().s("hello").to_string());
  assert_eq!("\x1b[36mhello", always().cyan().s("hello").to_string());
  assert_eq!("\x1b[37mhello", always().white().s("hello").to_string());

  assert_eq!("hello", never().black().s("hello").to_string());
  assert_eq!("hello", never().red().s("hello").to_string());
  assert_eq!("hello", never().green().s("hello").to_string());
  assert_eq!("hello", never().yellow().s("hello").to_string());
  assert_eq!("hello", never().blue().s("hello").to_string());
  assert_eq!("hello", never().magenta().s("hello").to_string());
  assert_eq!("hello", never().cyan().s("hello").to_string());
  assert_eq!("hello", never().white().s("hello").to_string());
}

#[test]
fn bright_colors_should_work() {
  assert_eq!("\x1b[90mhello", always().bright_black().s("hello").to_string());
  assert_eq!("\x1b[91mhello", always().bright_red().s("hello").to_string());
  assert_eq!("\x1b[92mhello", always().bright_green().s("hello").to_string());
  assert_eq!("\x1b[93mhello", always().bright_yellow().s("hello").to_string());
  assert_eq!("\x1b[94mhello", always().bright_blue().s("hello").to_string());
  assert_eq!("\x1b[95mhello", always().bright_magenta().s("hello").to_string());
  assert_eq!("\x1b[96mhello", always().bright_cyan().s("hello").to_string());
  assert_eq!("\x1b[97mhello", always().bright_white().s("hello").to_string());

  assert_eq!("hello", never().bright_black().s("hello").to_string());
  assert_eq!("hello", never().bright_red().s("hello").to_string());
  assert_eq!("hello", never().bright_green().s("hello").to_string());
  assert_eq!("hello", never().bright_yellow().s("hello").to_string());
  assert_eq!("hello", never().bright_blue().s("hello").to_string());
  assert_eq!("hello", never().bright_magenta().s("hello").to_string());
  assert_eq!("hello", never().bright_cyan().s("hello").to_string());
  assert_eq!("hello", never().bright_white().s("hello").to_string());
}

#[test]
fn bg_colors_should_work() {
  assert_eq!("\x1b[40mhello", always().bg_black().s("hello").to_string());
  assert_eq!("\x1b[41mhello", always().bg_red().s("hello").to_string());
  assert_eq!("\x1b[42mhello", always().bg_green().s("hello").to_string());
  assert_eq!("\x1b[43mhello", always().bg_yellow().s("hello").to_string());
  assert_eq!("\x1b[44mhello", always().bg_blue().s("hello").to_string());
  assert_eq!("\x1b[45mhello", always().bg_magenta().s("hello").to_string());
  assert_eq!("\x1b[46mhello", always().bg_cyan().s("hello").to_string());
  assert_eq!("\x1b[47mhello", always().bg_white().s("hello").to_string());

  assert_eq!("hello", never().bg_black().s("hello").to_string());
  assert_eq!("hello", never().bg_red().s("hello").to_string());
  assert_eq!("hello", never().bg_green().s("hello").to_string());
  assert_eq!("hello", never().bg_yellow().s("hello").to_string());
  assert_eq!("hello", never().bg_blue().s("hello").to_string());
  assert_eq!("hello", never().bg_magenta().s("hello").to_string());
  assert_eq!("hello", never().bg_cyan().s("hello").to_string());
  assert_eq!("hello", never().bg_white().s("hello").to_string());
}

#[test]
fn bg_bright_colors_should_work() {
  assert_eq!("\x1b[100mhello", always().bg_bright_black().s("hello").to_string());
  assert_eq!("\x1b[101mhello", always().bg_bright_red().s("hello").to_string());
  assert_eq!("\x1b[102mhello", always().bg_bright_green().s("hello").to_string());
  assert_eq!("\x1b[103mhello", always().bg_bright_yellow().s("hello").to_string());
  assert_eq!("\x1b[104mhello", always().bg_bright_blue().s("hello").to_string());
  assert_eq!("\x1b[105mhello", always().bg_bright_magenta().s("hello").to_string());
  assert_eq!("\x1b[106mhello", always().bg_bright_cyan().s("hello").to_string());
  assert_eq!("\x1b[107mhello", always().bg_bright_white().s("hello").to_string());

  assert_eq!("hello", never().bg_bright_black().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_red().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_green().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_yellow().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_blue().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_magenta().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_cyan().s("hello").to_string());
  assert_eq!("hello", never().bg_bright_white().s("hello").to_string());
}

#[test]
fn named_colors_should_work() {
  assert_eq!("\x1b[30mhello", always().color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[31mhello", always().color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[32mhello", always().color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[33mhello", always().color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[34mhello", always().color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[35mhello", always().color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[36mhello", always().color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[37mhello", always().color(Color::White).s("hello").to_string());
  assert_eq!("\x1b[38;5;132mhello", always().color(Color::Long(132)).s("hello").to_string());
  assert_eq!("\x1b[38;2;18;34;72mhello", always().color(Color::Rgb((18, 34, 72))).s("hello").to_string());

  assert_eq!("hello", never().color(Color::Black).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Red).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Green).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", never().color(Color::White).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Long(132)).s("hello").to_string());
  assert_eq!("hello", never().color(Color::Rgb((18, 34, 72))).s("hello").to_string());
}

#[test]
fn named_bright_colors_should_work() {
  assert_eq!("\x1b[90mhello", always().bright_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[91mhello", always().bright_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[92mhello", always().bright_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[93mhello", always().bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[94mhello", always().bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[95mhello", always().bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[96mhello", always().bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[97mhello", always().bright_color(Color::White).s("hello").to_string());

  assert_eq!("hello", never().bright_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", never().bright_color(Color::White).s("hello").to_string());
}

#[test]
fn named_bg_colors_should_work() {
  assert_eq!("\x1b[40mhello", always().bg_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[41mhello", always().bg_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[42mhello", always().bg_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[43mhello", always().bg_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[44mhello", always().bg_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[45mhello", always().bg_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[46mhello", always().bg_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[47mhello", always().bg_color(Color::White).s("hello").to_string());
  assert_eq!("\x1b[48;5;132mhello", always().bg_color(Color::Long(132)).s("hello").to_string());
  assert_eq!("\x1b[48;2;18;34;72mhello", always().bg_color(Color::Rgb((18, 34, 72))).s("hello").to_string());

  assert_eq!("hello", never().bg_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::White).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Long(132)).s("hello").to_string());
  assert_eq!("hello", never().bg_color(Color::Rgb((18, 34, 72))).s("hello").to_string());
}

#[test]
fn named_bg_bright_colors_should_work() {
  assert_eq!("\x1b[100mhello", always().bg_bright_color(Color::Black).s("hello").to_string());
  assert_eq!("\x1b[101mhello", always().bg_bright_color(Color::Red).s("hello").to_string());
  assert_eq!("\x1b[102mhello", always().bg_bright_color(Color::Green).s("hello").to_string());
  assert_eq!("\x1b[103mhello", always().bg_bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("\x1b[104mhello", always().bg_bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("\x1b[105mhello", always().bg_bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("\x1b[106mhello", always().bg_bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("\x1b[107mhello", always().bg_bright_color(Color::White).s("hello").to_string());

  assert_eq!("hello", never().bg_bright_color(Color::Black).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Red).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Green).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Yellow).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Blue).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Magenta).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::Cyan).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color(Color::White).s("hello").to_string());
}

#[test]
fn color_8_should_work() {
  assert_eq!("\x1b[30mhello", always().color_8(0).s("hello").to_string());
  assert_eq!("\x1b[31mhello", always().color_8(1).s("hello").to_string());
  assert_eq!("\x1b[32mhello", always().color_8(2).s("hello").to_string());
  assert_eq!("\x1b[33mhello", always().color_8(3).s("hello").to_string());
  assert_eq!("\x1b[34mhello", always().color_8(4).s("hello").to_string());
  assert_eq!("\x1b[35mhello", always().color_8(5).s("hello").to_string());
  assert_eq!("\x1b[36mhello", always().color_8(6).s("hello").to_string());
  assert_eq!("\x1b[37mhello", always().color_8(7).s("hello").to_string());
  assert_eq!("hello", always().color_8(8).s("hello").to_string());

  assert_eq!("hello", never().color_8(0).s("hello").to_string());
  assert_eq!("hello", never().color_8(1).s("hello").to_string());
  assert_eq!("hello", never().color_8(2).s("hello").to_string());
  assert_eq!("hello", never().color_8(3).s("hello").to_string());
  assert_eq!("hello", never().color_8(4).s("hello").to_string());
  assert_eq!("hello", never().color_8(5).s("hello").to_string());
  assert_eq!("hello", never().color_8(6).s("hello").to_string());
  assert_eq!("hello", never().color_8(7).s("hello").to_string());
  assert_eq!("hello", never().color_8(8).s("hello").to_string());
}

#[test]
fn bright_color_8_should_work() {
  assert_eq!("\x1b[91mhello", always().bright_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[92mhello", always().bright_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[90mhello", always().bright_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[93mhello", always().bright_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[94mhello", always().bright_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[95mhello", always().bright_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[96mhello", always().bright_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[97mhello", always().bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", always().bright_color_8(8).s("hello").to_string());

  assert_eq!("hello", never().bright_color_8(0).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(1).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(2).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(3).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(4).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(5).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(6).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", never().bright_color_8(8).s("hello").to_string());
}

#[test]
fn bg_color_8_should_work() {
  assert_eq!("\x1b[40mhello", always().bg_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[41mhello", always().bg_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[42mhello", always().bg_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[43mhello", always().bg_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[44mhello", always().bg_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[45mhello", always().bg_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[46mhello", always().bg_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[47mhello", always().bg_color_8(7).s("hello").to_string());
  assert_eq!("hello", always().bg_color_8(8).s("hello").to_string());

  assert_eq!("hello", never().bg_color_8(0).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(1).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(2).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(3).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(4).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(5).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(6).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(7).s("hello").to_string());
  assert_eq!("hello", never().bg_color_8(8).s("hello").to_string());
}

#[test]
fn bg_bright_color_8_should_work() {
  assert_eq!("\x1b[100mhello", always().bg_bright_color_8(0).s("hello").to_string());
  assert_eq!("\x1b[101mhello", always().bg_bright_color_8(1).s("hello").to_string());
  assert_eq!("\x1b[102mhello", always().bg_bright_color_8(2).s("hello").to_string());
  assert_eq!("\x1b[103mhello", always().bg_bright_color_8(3).s("hello").to_string());
  assert_eq!("\x1b[104mhello", always().bg_bright_color_8(4).s("hello").to_string());
  assert_eq!("\x1b[105mhello", always().bg_bright_color_8(5).s("hello").to_string());
  assert_eq!("\x1b[106mhello", always().bg_bright_color_8(6).s("hello").to_string());
  assert_eq!("\x1b[107mhello", always().bg_bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", always().bg_bright_color_8(8).s("hello").to_string());

  assert_eq!("hello", never().bg_bright_color_8(0).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(1).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(2).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(3).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(4).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(5).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(6).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(7).s("hello").to_string());
  assert_eq!("hello", never().bg_bright_color_8(8).s("hello").to_string());
}

#[test]
fn color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\x1b[38;5;{}mhello", i), always().color_256(i).s("hello").to_string());
    assert_eq!("hello", never().color_256(i).s("hello").to_string());
  }
}

#[test]
fn bg_color_256_should_work() {
  for i in 0..=255 {
    assert_eq!(format!("\x1b[48;5;{}mhello", i), always().bg_color_256(i).s("hello").to_string());
    assert_eq!("hello", never().bg_color_256(i).s("hello").to_string());
  }
}

#[test]
fn color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\x1b[38;2;{};{};{}mhello", r, g, b), always().color_rgb((r, g, b)).s("hello").to_string());
        assert_eq!("hello", never().color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}

#[test]
fn bg_color_rgb_should_work() {
  for r in 1..=10 {
    for g in 20..=30 {
      for b in 50..=60 {
        assert_eq!(format!("\x1b[48;2;{};{};{}mhello", r, g, b), always().bg_color_rgb((r, g, b)).s("hello").to_string());
        assert_eq!("hello", never().bg_color_rgb((r, g, b)).s("hello").to_string());
      }
    }
  }
}

#[test]
fn color_conversion_should_work() {
  assert_eq!("\x1b[30mhello", always().color(0.into()).s("hello").to_string());
  assert_eq!("\x1b[31mhello", always().color(1.into()).s("hello").to_string());
  assert_eq!("\x1b[32mhello", always().color(2.into()).s("hello").to_string());
  assert_eq!("\x1b[33mhello", always().color(3.into()).s("hello").to_string());
  assert_eq!("\x1b[34mhello", always().color(4.into()).s("hello").to_string());
  assert_eq!("\x1b[35mhello", always().color(5.into()).s("hello").to_string());
  assert_eq!("\x1b[36mhello", always().color(6.into()).s("hello").to_string());
  assert_eq!("\x1b[37mhello", always().color(7.into()).s("hello").to_string());
  assert_eq!("\x1b[38;5;132mhello", always().color(132.into()).s("hello").to_string());
  assert_eq!("\x1b[38;2;10;73;134mhello", always().color((10, 73, 134).into()).s("hello").to_string());
}
