use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use std::fmt;
use std::fmt::{Alignment, Display, Write};
use std::ops::{Add, AddAssign};

/// A trait representing styled text.
pub trait StyledText {
  /// Adds content to text.
  fn s<T: Display>(self, s: T) -> Self;
  /// Resets all colors and styling flags.
  fn reset(self) -> Self;
  /// Adds repeated content to text.
  fn repeat<T: Display>(self, s: T, n: usize) -> Self;
  /// Adds `s` suffix to the content when the number is not 1.
  fn plural<T: Display>(self, s: T, n: usize) -> Self;
  /// Style text as bold.
  fn bold(self) -> Self;
  /// Style text as italic.
  fn italic(self) -> Self;
  /// Style text as underlined.
  fn underline(self) -> Self;
  /// Sets the foreground color to black.
  fn black(self) -> Self;
  /// Sets the foreground color to bright black.
  fn bright_black(self) -> Self;
  /// Sets the foreground color to red.
  fn red(self) -> Self;
  /// Sets the foreground color to bright red.
  fn bright_red(self) -> Self;
  /// Sets the foreground color to green.
  fn green(self) -> Self;
  /// Sets the foreground color to bright green.
  fn bright_green(self) -> Self;
  /// Sets the foreground color to yellow.
  fn yellow(self) -> Self;
  /// Sets the foreground color to bright yellow.
  fn bright_yellow(self) -> Self;
  /// Sets the foreground color to blue.
  fn blue(self) -> Self;
  /// Sets the foreground color to bright blue.
  fn bright_blue(self) -> Self;
  /// Sets the foreground color to magenta.
  fn magenta(self) -> Self;
  /// Sets the foreground color to bright magenta.
  fn bright_magenta(self) -> Self;
  /// Sets the foreground color to cyan.
  fn cyan(self) -> Self;
  /// Sets the foreground color to bright cyan.
  fn bright_cyan(self) -> Self;
  /// Sets the foreground color to white.
  fn white(self) -> Self;
  /// Sets the foreground color to bright white.
  fn bright_white(self) -> Self;
  /// Sets the background color to black.
  fn bg_black(self) -> Self;
  /// Sets the background color to bright black.
  fn bg_bright_black(self) -> Self;
  /// Sets the background color to red.
  fn bg_red(self) -> Self;
  /// Sets the background color to bright red.
  fn bg_bright_red(self) -> Self;
  /// Sets the background color to green.
  fn bg_green(self) -> Self;
  /// Sets the background color to bright green.
  fn bg_bright_green(self) -> Self;
  /// Sets the background color to yellow.
  fn bg_yellow(self) -> Self;
  /// Sets the background color to bright yellow.
  fn bg_bright_yellow(self) -> Self;
  /// Sets the background color to blue.
  fn bg_blue(self) -> Self;
  /// Sets the background color to bright blue.
  fn bg_bright_blue(self) -> Self;
  /// Sets the background color to magenta.
  fn bg_magenta(self) -> Self;
  /// Sets the background color to bright magenta.
  fn bg_bright_magenta(self) -> Self;
  /// Sets the background color to cyan.
  fn bg_cyan(self) -> Self;
  /// Sets the background color to bright cyan.
  fn bg_bright_cyan(self) -> Self;
  /// Sets the background color to white.
  fn bg_white(self) -> Self;
  /// Sets the background color to bright white.
  fn bg_bright_white(self) -> Self;
  /// Sets the color by color enumeration.
  fn color(self, c: Color) -> Self;
  /// Sets the bright color by color enumeration.
  fn bright_color(self, c: Color) -> Self;
  /// Sets the background color by color enumeration.
  fn bg_color(self, c: Color) -> Self;
  /// Sets the background bright color by color enumeration.
  fn bg_bright_color(self, c: Color) -> Self;
  /// Sets the color by color index.
  fn color_8(self, c: u8) -> Self;
  /// Sets the bright color by color index.
  fn bright_color_8(self, c: u8) -> Self;
  /// Sets the background color by color index.
  fn bg_color_8(self, c: u8) -> Self;
  /// Sets the background bright color by color index.
  fn bg_bright_color_8(self, c: u8) -> Self;
  fn color_256(self, c: u8) -> Self;
  fn bg_color_256(self, c: u8) -> Self;
  fn color_rgb(self, c: RgbColor) -> Self;
  fn bg_color_rgb(self, c: RgbColor) -> Self;
}

/// Styled text.
#[derive(Debug, Clone)]
pub struct Text {
  /// Color mode.
  cm: ColorMode,
  /// Text content.
  content: String,
  /// Number of characters in the text.
  chars: usize,
}

impl Display for Text {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(width) = f.width() {
      let fill = width.saturating_sub(self.chars);
      let ch = f.fill().to_string();
      if let Some(align) = f.align() {
        match align {
          Alignment::Left => write!(f, "{}{}", self.content, ch.repeat(fill)),
          Alignment::Right => write!(f, "{}{}", ch.repeat(fill), self.content),
          Alignment::Center => write!(f, "{}{}{}", ch.repeat(fill / 2), self.content, ch.repeat(fill - fill / 2)),
        }
      } else {
        write!(f, "{}{}", self.content, ch.repeat(fill))
      }
    } else {
      write!(f, "{}", self.content)
    }
  }
}

impl Default for Text {
  fn default() -> Self {
    Self::new(ColorMode::default())
  }
}

impl From<ColorMode> for Text {
  fn from(cm: ColorMode) -> Self {
    Self::new(cm)
  }
}

impl Text {
  pub fn new(cm: ColorMode) -> Self {
    Self {
      cm,
      content: String::default(),
      chars: 0,
    }
  }

  pub fn auto() -> Self {
    Self {
      cm: ColorMode::default(),
      content: String::default(),
      chars: 0,
    }
  }

  pub fn on() -> Self {
    Self {
      cm: ColorMode::On,
      content: String::default(),
      chars: 0,
    }
  }

  pub fn off() -> Self {
    Self {
      cm: ColorMode::Off,
      content: String::default(),
      chars: 0,
    }
  }
}

impl StyledText for Text {
  fn s<T: Display>(mut self, s: T) -> Self {
    let buffer = format!("{}", s);
    self.content.reserve(buffer.len());
    self.content.push_str(&buffer);
    self.chars += buffer.chars().count();
    self
  }

  fn reset(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.reset());
    self
  }

  fn repeat<T: Display>(self, s: T, n: usize) -> Self {
    self.s(s.to_string().repeat(n))
  }

  fn plural<T: Display>(self, s: T, n: usize) -> Self {
    if n == 1 { self.s(s) } else { self.s(format!("{}s", s)) }
  }

  fn bold(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bold());
    self
  }

  fn italic(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.italic());
    self
  }

  fn underline(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.underline());
    self
  }

  fn black(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.black(false));
    self
  }

  fn bright_black(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.black(true));
    self
  }

  fn red(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.red(false));
    self
  }

  fn bright_red(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.red(true));
    self
  }

  fn green(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.green(false));
    self
  }

  fn bright_green(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.green(true));
    self
  }

  fn yellow(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.yellow(false));
    self
  }

  fn bright_yellow(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.yellow(true));
    self
  }

  fn blue(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.blue(false));
    self
  }

  fn bright_blue(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.blue(true));
    self
  }

  fn magenta(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.magenta(false));
    self
  }

  fn bright_magenta(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.magenta(true));
    self
  }

  fn cyan(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.cyan(false));
    self
  }

  fn bright_cyan(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.cyan(true));
    self
  }

  fn white(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.white(false));
    self
  }

  fn bright_white(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.white(true));
    self
  }

  fn bg_black(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_black(false));
    self
  }

  fn bg_bright_black(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_black(true));
    self
  }

  fn bg_red(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_red(false));
    self
  }

  fn bg_bright_red(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_red(true));
    self
  }

  fn bg_green(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_green(false));
    self
  }

  fn bg_bright_green(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_green(true));
    self
  }

  fn bg_yellow(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_yellow(false));
    self
  }

  fn bg_bright_yellow(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_yellow(true));
    self
  }

  fn bg_blue(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_blue(false));
    self
  }

  fn bg_bright_blue(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_blue(true));
    self
  }

  fn bg_magenta(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_magenta(false));
    self
  }

  fn bg_bright_magenta(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_magenta(true));
    self
  }

  fn bg_cyan(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_cyan(false));
    self
  }

  fn bg_bright_cyan(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_cyan(true));
    self
  }

  fn bg_white(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_white(false));
    self
  }

  fn bg_bright_white(mut self) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_white(true));
    self
  }

  fn color(mut self, c: Color) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color(c, false));
    self
  }

  fn bright_color(mut self, c: Color) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color(c, true));
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color(c, false));
    self
  }

  fn bg_bright_color(mut self, c: Color) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color(c, true));
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color_8(c, false));
    self
  }

  fn bright_color_8(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color_8(c, true));
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color_8(c, false));
    self
  }

  fn bg_bright_color_8(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color_8(c, true));
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color_256(c));
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color_256(c));
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.color_rgb(c));
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    _ = write!(&mut self.content, "{}", self.cm.bg_color_rgb(c));
    self
  }
}

impl AddAssign<Text> for Text {
  fn add_assign(&mut self, rhs: Text) {
    *self += &rhs;
  }
}

impl AddAssign<&Text> for Text {
  fn add_assign(&mut self, rhs: &Text) {
    self.content.reserve(rhs.content.len());
    self.content.push_str(&rhs.content);
    self.chars += rhs.chars;
  }
}

impl AddAssign<&str> for Text {
  fn add_assign(&mut self, rhs: &str) {
    self.content.reserve(rhs.len());
    self.content.push_str(rhs);
    self.chars += rhs.chars().count();
  }
}

impl Add<Text> for Text {
  type Output = Text;
  fn add(mut self, rhs: Text) -> Text {
    self += rhs;
    self
  }
}

impl Add<&Text> for Text {
  type Output = Text;
  fn add(mut self, rhs: &Text) -> Text {
    self += rhs;
    self
  }
}

impl Add<&str> for Text {
  type Output = Text;
  fn add(mut self, rhs: &str) -> Text {
    self += rhs;
    self
  }
}

impl Add<Text> for &str {
  type Output = Text;
  fn add(self, rhs: Text) -> Text {
    self.add(&rhs)
  }
}

impl Add<&Text> for &str {
  type Output = Text;
  fn add(self, rhs: &Text) -> Text {
    let mut content = String::with_capacity(self.len() + rhs.content.len());
    content.push_str(self);
    content.push_str(&rhs.content);
    Text {
      cm: rhs.cm,
      content,
      chars: self.chars().count() + rhs.chars,
    }
  }
}

pub fn auto() -> Text {
  Text::auto()
}

pub fn always() -> Text {
  Text::on()
}

pub fn never() -> Text {
  Text::off()
}
