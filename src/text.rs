use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use std::fmt;
use std::fmt::{Alignment, Display, Write};
use std::ops::Add;

/// A trait representing styled text.
pub trait StyledText {
  /// Adds content to text.
  fn s<T: Display>(self, s: T) -> Self;
  /// Resets (clears) all styling flags.
  fn normal(self) -> Self;
  /// Adds repeated content to text.
  fn repeat<T: Display>(self, s: T, n: usize) -> Self;
  /// Adds -s suffix to the content when the number is not 1.
  fn plural<T: Display>(self, s: T, n: usize) -> Self;
  /// Style text as bold.
  fn bold(self) -> Self;
  /// Style text as italic.
  fn italic(self) -> Self;
  /// Style text as underlined.
  fn underline(self) -> Self;
  /// Sets the foreground color to black.
  fn black(self) -> Self;
  /// Sets the foreground color to red.
  fn red(self) -> Self;
  /// Sets the foreground color to green.
  fn green(self) -> Self;
  /// Sets the foreground color to yellow.
  fn yellow(self) -> Self;
  /// Sets the foreground color to blue.
  fn blue(self) -> Self;
  /// Sets the foreground color to magenta.
  fn magenta(self) -> Self;
  /// Sets the foreground color to cyan.
  fn cyan(self) -> Self;
  /// Sets the foreground color to white.
  fn white(self) -> Self;
  fn bg_black(self) -> Self;
  fn bg_red(self) -> Self;
  fn bg_green(self) -> Self;
  fn bg_yellow(self) -> Self;
  fn bg_blue(self) -> Self;
  fn bg_magenta(self) -> Self;
  fn bg_cyan(self) -> Self;
  fn bg_white(self) -> Self;
  fn color(self, c: Color) -> Self;
  fn bg_color(self, c: Color) -> Self;
  fn color_8(self, c: u8) -> Self;
  fn bg_color_8(self, c: u8) -> Self;
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
  /// Text length.
  length: usize,
}

impl Display for Text {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(width) = f.width() {
      let fill = width.saturating_sub(self.length);
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
      length: 0,
    }
  }

  pub fn auto() -> Self {
    Self {
      cm: ColorMode::default(),
      content: String::default(),
      length: 0,
    }
  }

  pub fn on() -> Self {
    Self {
      cm: ColorMode::On,
      content: String::default(),
      length: 0,
    }
  }

  pub fn off() -> Self {
    Self {
      cm: ColorMode::Off,
      content: String::default(),
      length: 0,
    }
  }
}

impl StyledText for Text {
  fn s<T: Display>(mut self, s: T) -> Self {
    let length = self.content.len();
    let _ = write!(&mut self.content, "{}", s);
    self.length += self.content.len() - length;
    self
  }

  fn normal(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.clear());
    self
  }

  fn repeat<T: Display>(self, s: T, n: usize) -> Self {
    self.s(s.to_string().repeat(n))
  }

  fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
    let _ = if n == 1 {
      write!(&mut self.content, "{}", s)
    } else {
      write!(&mut self.content, "{}s", s)
    };
    self
  }

  fn bold(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bold());
    self
  }

  fn italic(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.italic());
    self
  }

  fn underline(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.underline());
    self
  }

  fn black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.black());
    self
  }

  fn red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.red());
    self
  }

  fn green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.green());
    self
  }

  fn yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.yellow());
    self
  }

  fn blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.blue());
    self
  }

  fn magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.magenta());
    self
  }

  fn cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.cyan());
    self
  }

  fn white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.white());
    self
  }

  fn bg_black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_black());
    self
  }

  fn bg_red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_red());
    self
  }

  fn bg_green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_green());
    self
  }

  fn bg_yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_yellow());
    self
  }

  fn bg_blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_blue());
    self
  }

  fn bg_magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_magenta());
    self
  }

  fn bg_cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_cyan());
    self
  }

  fn bg_white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_white());
    self
  }

  fn color(mut self, c: Color) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.color(c));
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_color(c));
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.color_8(c));
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_color_8(c));
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.color_256(c));
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_color_256(c));
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.color_rgb(c));
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    let _ = write!(&mut self.content, "{}", self.cm.bg_color_rgb(c));
    self
  }
}

impl Add for Text {
  type Output = Self;

  /// Concatenates styled texts.
  fn add(self, rhs: Self) -> Self::Output {
    let mut content = self.content;
    content.push_str(&rhs.content);
    Self {
      cm: self.cm,
      content,
      length: self.length + rhs.length,
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
