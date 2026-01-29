//! # Terminal color sequences

/// Type alias for RGB color.
pub type RgbColor = (u8, u8, u8);

/// Type representing a color value in several formats.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
  None,
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
