//! # Color mode

use crate::{Color, RgbColor};
use std::borrow::Cow;
use std::io::IsTerminal;

/// Color mode for switching coloring on/off.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorMode {
  /// Switch coloring **on**.
  On,
  /// Switch coloring **off**.
  Off,
}

impl Default for ColorMode {
  fn default() -> Self {
    if std::io::stdout().is_terminal() { Self::On } else { Self::Off }
  }
}

impl From<&str> for ColorMode {
  /// Creates [ColorMode] from str reference.
  fn from(value: &str) -> Self {
    Self::new(value)
  }
}

impl From<String> for ColorMode {
  /// Creates [ColorMode] from string.
  fn from(value: String) -> Self {
    Self::new(value)
  }
}

impl From<&String> for ColorMode {
  /// Creates [ColorMode] from string reference.
  fn from(value: &String) -> Self {
    Self::new(value)
  }
}

impl From<Option<String>> for ColorMode {
  /// Creates [ColorMode] from optional string.
  fn from(value: Option<String>) -> Self {
    value.map_or(Self::default(), Self::new)
  }
}

impl From<Option<&String>> for ColorMode {
  /// Creates [ColorMode] from optional string reference.
  fn from(value: Option<&String>) -> Self {
    value.map_or(Self::default(), Self::new)
  }
}

impl ColorMode {
  pub fn new(s: impl AsRef<str>) -> Self {
    let s = s.as_ref();
    if s.eq_ignore_ascii_case("never") {
      return Self::Off;
    }
    if s.eq_ignore_ascii_case("always") {
      return Self::On;
    }
    Self::default()
  }

  pub fn black(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(0, bright)
  }

  pub fn red(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(1, bright)
  }

  pub fn green(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(2, bright)
  }

  pub fn yellow(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(3, bright)
  }

  pub fn blue(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(4, bright)
  }

  pub fn magenta(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(5, bright)
  }

  pub fn cyan(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(6, bright)
  }

  pub fn white(&self, bright: bool) -> Cow<'static, str> {
    self.color_8(7, bright)
  }

  pub fn bg_black(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(0, bright)
  }

  pub fn bg_red(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(1, bright)
  }

  pub fn bg_green(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(2, bright)
  }

  pub fn bg_yellow(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(3, bright)
  }

  pub fn bg_blue(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(4, bright)
  }

  pub fn bg_magenta(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(5, bright)
  }

  pub fn bg_cyan(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(6, bright)
  }

  pub fn bg_white(&self, bright: bool) -> Cow<'static, str> {
    self.bg_color_8(7, bright)
  }

  pub fn color(&self, c: Color, bright: bool) -> Cow<'static, str> {
    match c {
      Color::None => Cow::Borrowed(""),
      Color::Black => self.color_8(0, bright),
      Color::Red => self.color_8(1, bright),
      Color::Green => self.color_8(2, bright),
      Color::Yellow => self.color_8(3, bright),
      Color::Blue => self.color_8(4, bright),
      Color::Magenta => self.color_8(5, bright),
      Color::Cyan => self.color_8(6, bright),
      Color::White => self.color_8(7, bright),
      Color::Long(value) => Cow::Owned(self.color_256(value)),
      Color::Rgb(value) => Cow::Owned(self.color_rgb(value)),
    }
  }

  pub fn bg_color(&self, c: Color, bright: bool) -> Cow<'static, str> {
    match c {
      Color::None => Cow::Borrowed(""),
      Color::Black => self.bg_color_8(0, bright),
      Color::Red => self.bg_color_8(1, bright),
      Color::Green => self.bg_color_8(2, bright),
      Color::Yellow => self.bg_color_8(3, bright),
      Color::Blue => self.bg_color_8(4, bright),
      Color::Magenta => self.bg_color_8(5, bright),
      Color::Cyan => self.bg_color_8(6, bright),
      Color::White => self.bg_color_8(7, bright),
      Color::Long(value) => Cow::Owned(self.bg_color_256(value)),
      Color::Rgb(value) => Cow::Owned(self.bg_color_rgb(value)),
    }
  }

  pub fn color_8(&self, c: u8, bright: bool) -> Cow<'static, str> {
    match self {
      ColorMode::On => match (bright, c) {
        (false, 0) => Cow::Borrowed("\x1b[30m"),
        (false, 1) => Cow::Borrowed("\x1b[31m"),
        (false, 2) => Cow::Borrowed("\x1b[32m"),
        (false, 3) => Cow::Borrowed("\x1b[33m"),
        (false, 4) => Cow::Borrowed("\x1b[34m"),
        (false, 5) => Cow::Borrowed("\x1b[35m"),
        (false, 6) => Cow::Borrowed("\x1b[36m"),
        (false, 7) => Cow::Borrowed("\x1b[37m"),
        (true, 0) => Cow::Borrowed("\x1b[90m"),
        (true, 1) => Cow::Borrowed("\x1b[91m"),
        (true, 2) => Cow::Borrowed("\x1b[92m"),
        (true, 3) => Cow::Borrowed("\x1b[93m"),
        (true, 4) => Cow::Borrowed("\x1b[94m"),
        (true, 5) => Cow::Borrowed("\x1b[95m"),
        (true, 6) => Cow::Borrowed("\x1b[96m"),
        (true, 7) => Cow::Borrowed("\x1b[97m"),
        _ => Cow::Borrowed(""),
      },
      _ => Cow::Borrowed(""),
    }
  }

  pub fn bg_color_8(&self, c: u8, bright: bool) -> Cow<'static, str> {
    match self {
      ColorMode::On => match (bright, c) {
        (false, 0) => Cow::Borrowed("\x1b[40m"),
        (false, 1) => Cow::Borrowed("\x1b[41m"),
        (false, 2) => Cow::Borrowed("\x1b[42m"),
        (false, 3) => Cow::Borrowed("\x1b[43m"),
        (false, 4) => Cow::Borrowed("\x1b[44m"),
        (false, 5) => Cow::Borrowed("\x1b[45m"),
        (false, 6) => Cow::Borrowed("\x1b[46m"),
        (false, 7) => Cow::Borrowed("\x1b[47m"),
        (true, 0) => Cow::Borrowed("\x1b[100m"),
        (true, 1) => Cow::Borrowed("\x1b[101m"),
        (true, 2) => Cow::Borrowed("\x1b[102m"),
        (true, 3) => Cow::Borrowed("\x1b[103m"),
        (true, 4) => Cow::Borrowed("\x1b[104m"),
        (true, 5) => Cow::Borrowed("\x1b[105m"),
        (true, 6) => Cow::Borrowed("\x1b[106m"),
        (true, 7) => Cow::Borrowed("\x1b[107m"),
        _ => Cow::Borrowed(""),
      },
      _ => Cow::Borrowed(""),
    }
  }

  pub fn color_256(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\x1b[38;5;{}m", c),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_256(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\x1b[48;5;{}m", c),
      _ => "".to_string(),
    }
  }

  pub fn color_rgb(&self, c: RgbColor) -> String {
    match self {
      ColorMode::On => format!("\x1b[38;2;{};{};{}m", c.0, c.1, c.2),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_rgb(&self, c: RgbColor) -> String {
    match self {
      ColorMode::On => format!("\x1b[48;2;{};{};{}m", c.0, c.1, c.2),
      _ => "".to_string(),
    }
  }

  pub fn bold(&self) -> &str {
    match self {
      ColorMode::On => "\x1b[1m",
      _ => "",
    }
  }

  pub fn italic(&self) -> &str {
    match self {
      ColorMode::On => "\x1b[3m",
      _ => "",
    }
  }

  pub fn underline(&self) -> &str {
    match self {
      ColorMode::On => "\x1b[4m",
      _ => "",
    }
  }

  pub fn reset(&self) -> &str {
    match self {
      ColorMode::On => "\x1b[0m",
      _ => "",
    }
  }
}
