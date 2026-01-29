use crate::colors::{Color, ColorMode, RgbColor};
use std::fmt;
use std::fmt::{Display, Write};
use std::ops::Add;

pub trait StyledText {
  /// Adds content to text.
  fn s<T: Display>(self, s: T) -> Self;
  /// Clears all styling flags.
  fn c(self) -> Self;
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

#[derive(Debug, Clone)]
pub struct Text {
  color_mode: ColorMode,
  content: String,
}

impl Display for Text {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)
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
  pub fn new(color_mode: ColorMode) -> Self {
    Self {
      color_mode,
      content: String::default(),
    }
  }

  pub fn print(&self) {
    print!("{}", self.content);
  }

  pub fn printc(&self) {
    print!("{}{}", self.content, self.color_mode.clear());
  }

  pub fn println(&self) {
    println!("{}", self.content);
  }

  pub fn printlnc(&self) {
    println!("{}{}", self.content, self.color_mode.clear());
  }

  pub fn eprint(&self) {
    eprint!("{}", self.content);
  }

  pub fn eprintc(&self) {
    eprint!("{}{}", self.content, self.color_mode.clear());
  }

  pub fn eprintln(&self) {
    eprintln!("{}", self.content);
  }

  pub fn eprintlnc(&self) {
    eprintln!("{}{}", self.content, self.color_mode.clear());
  }
}

impl StyledText for Text {
  fn s<T: Display>(mut self, s: T) -> Self {
    let _ = write!(&mut self.content, "{}", s);
    self
  }

  fn c(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.clear());
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
    let _ = write!(&mut self.content, "{}", self.color_mode.bold());
    self
  }

  fn italic(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.italic());
    self
  }

  fn underline(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.underline());
    self
  }

  fn black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.black());
    self
  }

  fn red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.red());
    self
  }

  fn green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.green());
    self
  }

  fn yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.yellow());
    self
  }

  fn blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.blue());
    self
  }

  fn magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.magenta());
    self
  }

  fn cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.cyan());
    self
  }

  fn white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.white());
    self
  }

  fn bg_black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_black());
    self
  }

  fn bg_red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_red());
    self
  }

  fn bg_green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_green());
    self
  }

  fn bg_yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_yellow());
    self
  }

  fn bg_blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_blue());
    self
  }

  fn bg_magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_magenta());
    self
  }

  fn bg_cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_cyan());
    self
  }

  fn bg_white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_white());
    self
  }

  fn color(mut self, c: Color) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color(c));
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color(c));
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color_8(c));
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color_8(c));
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color_256(c));
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color_256(c));
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color_rgb(c));
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color_rgb(c));
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
      color_mode: self.color_mode,
      content,
    }
  }
}
