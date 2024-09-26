//! # Terminal color sequences

use std::io::IsTerminal;

/// Type alias for RGB color.
pub type RgbColor = (u8, u8, u8);

/// Type representing a color value in several formats.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  Long(u8),
  Rgb(RgbColor),
}

impl From<u8> for Color {
  fn from(n: u8) -> Self {
    match n {
      0 => Self::Black,
      1 => Self::Red,
      2 => Self::Green,
      3 => Self::Yellow,
      4 => Self::Blue,
      5 => Self::Magenta,
      6 => Self::Cyan,
      7 => Self::White,
      _ => Self::Long(n),
    }
  }
}

impl From<RgbColor> for Color {
  fn from(n: RgbColor) -> Self {
    Self::Rgb(n)
  }
}

/// Color mode to switch terminal colouring `ON` or `OFF`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorMode {
  /// Switch colouring **on**.
  On,
  /// Switch colouring **off**.
  Off,
}

impl Default for ColorMode {
  fn default() -> Self {
    if std::io::stdout().is_terminal() {
      Self::On
    } else {
      Self::Off
    }
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
    Self::new(&value)
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
    value.map_or(Self::default(), |s| Self::new(&s))
  }
}

impl From<Option<&String>> for ColorMode {
  /// Creates [ColorMode] from optional string reference.
  fn from(value: Option<&String>) -> Self {
    value.map_or(Self::default(), |s| Self::new(s))
  }
}

impl ColorMode {
  pub fn new(s: &str) -> Self {
    match s.to_lowercase().trim() {
      "never" => Self::Off,
      "always" => Self::On,
      _ => Self::default(),
    }
  }

  pub fn black(&self) -> String {
    self.color_8(0)
  }

  pub fn red(&self) -> String {
    self.color_8(1)
  }

  pub fn green(&self) -> String {
    self.color_8(2)
  }

  pub fn yellow(&self) -> String {
    self.color_8(3)
  }

  pub fn blue(&self) -> String {
    self.color_8(4)
  }

  pub fn magenta(&self) -> String {
    self.color_8(5)
  }

  pub fn cyan(&self) -> String {
    self.color_8(6)
  }

  pub fn white(&self) -> String {
    self.color_8(7)
  }

  pub fn bg_black(&self) -> String {
    self.bg_color_8(0)
  }

  pub fn bg_red(&self) -> String {
    self.bg_color_8(1)
  }

  pub fn bg_green(&self) -> String {
    self.bg_color_8(2)
  }

  pub fn bg_yellow(&self) -> String {
    self.bg_color_8(3)
  }

  pub fn bg_blue(&self) -> String {
    self.bg_color_8(4)
  }

  pub fn bg_magenta(&self) -> String {
    self.bg_color_8(5)
  }

  pub fn bg_cyan(&self) -> String {
    self.bg_color_8(6)
  }

  pub fn bg_white(&self) -> String {
    self.bg_color_8(7)
  }

  pub fn color(&self, c: Color) -> String {
    match c {
      Color::Black => self.color_8(0),
      Color::Red => self.color_8(1),
      Color::Green => self.color_8(2),
      Color::Yellow => self.color_8(3),
      Color::Blue => self.color_8(4),
      Color::Magenta => self.color_8(5),
      Color::Cyan => self.color_8(6),
      Color::White => self.color_8(7),
      Color::Long(value) => self.color_256(value),
      Color::Rgb(value) => self.color_rgb(value),
    }
  }

  pub fn bg_color(&self, c: Color) -> String {
    match c {
      Color::Black => self.bg_color_8(0),
      Color::Red => self.bg_color_8(1),
      Color::Green => self.bg_color_8(2),
      Color::Yellow => self.bg_color_8(3),
      Color::Blue => self.bg_color_8(4),
      Color::Magenta => self.bg_color_8(5),
      Color::Cyan => self.bg_color_8(6),
      Color::White => self.bg_color_8(7),
      Color::Long(value) => self.bg_color_256(value),
      Color::Rgb(value) => self.bg_color_rgb(value),
    }
  }

  pub fn color_8(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[{}m", 30 + c.clamp(0, 7)),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_8(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[{}m", 40 + c.clamp(0, 7)),
      _ => "".to_string(),
    }
  }

  pub fn color_256(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[38;5;{}m", c),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_256(&self, c: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[48;5;{}m", c),
      _ => "".to_string(),
    }
  }

  pub fn color_rgb(&self, c: RgbColor) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[38;2;{};{};{}m", c.0, c.1, c.2),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_rgb(&self, c: RgbColor) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[48;2;{};{};{}m", c.0, c.1, c.2),
      _ => "".to_string(),
    }
  }

  pub fn bold(&self) -> &str {
    match self {
      ColorMode::On => "\u{1b}[1m",
      _ => "",
    }
  }

  pub fn italic(&self) -> &str {
    match self {
      ColorMode::On => "\u{1b}[3m",
      _ => "",
    }
  }

  pub fn underline(&self) -> &str {
    match self {
      ColorMode::On => "\u{1b}[4m",
      _ => "",
    }
  }

  pub fn clear(&self) -> &str {
    match self {
      ColorMode::On => "\u{1b}[0m",
      _ => "",
    }
  }
}
