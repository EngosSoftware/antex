//! # Terminal color sequences

use std::io::IsTerminal;

pub const BLACK: u8 = 0;
pub const RED: u8 = 1;
pub const GREEN: u8 = 2;
pub const YELLOW: u8 = 3;
pub const BLUE: u8 = 4;
pub const MAGENTA: u8 = 5;
pub const CYAN: u8 = 6;
pub const WHITE: u8 = 7;

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
    self.color(0)
  }

  pub fn red(&self) -> String {
    self.color(1)
  }

  pub fn green(&self) -> String {
    self.color(2)
  }

  pub fn yellow(&self) -> String {
    self.color(3)
  }

  pub fn blue(&self) -> String {
    self.color(4)
  }

  pub fn magenta(&self) -> String {
    self.color(5)
  }

  pub fn cyan(&self) -> String {
    self.color(6)
  }

  pub fn white(&self) -> String {
    self.color(7)
  }

  pub fn bg_black(&self) -> String {
    self.bg_color(0)
  }

  pub fn bg_red(&self) -> String {
    self.bg_color(1)
  }

  pub fn bg_green(&self) -> String {
    self.bg_color(2)
  }

  pub fn bg_yellow(&self) -> String {
    self.bg_color(3)
  }

  pub fn bg_blue(&self) -> String {
    self.bg_color(4)
  }

  pub fn bg_magenta(&self) -> String {
    self.bg_color(5)
  }

  pub fn bg_cyan(&self) -> String {
    self.bg_color(6)
  }

  pub fn bg_white(&self) -> String {
    self.bg_color(7)
  }

  pub fn color(&self, value: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[{}m", 30 + value.clamp(0, 7)),
      _ => "".to_string(),
    }
  }

  pub fn bg_color(&self, value: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[{}m", 40 + value.clamp(0, 7)),
      _ => "".to_string(),
    }
  }

  pub fn color_256(&self, value: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[38;5;{}m", value),
      _ => "".to_string(),
    }
  }

  pub fn bg_color_256(&self, value: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[48;5;{}m", value),
      _ => "".to_string(),
    }
  }

  pub fn rgb(&self, r: u8, g: u8, b: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[38;2;{};{};{}m", r, g, b),
      _ => "".to_string(),
    }
  }

  pub fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
    match self {
      ColorMode::On => format!("\u{1b}[48;2;{};{};{}m", r, g, b),
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
