use crate::colors::ColorMode;
use std::fmt::{Display, Write};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Text {
  color_mode: ColorMode,
  content: String,
}

impl std::fmt::Display for Text {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

  pub fn s<T: Display>(mut self, s: T) -> Self {
    let _ = write!(&mut self.content, "{}", s);
    self
  }

  pub fn nl(self) -> Self {
    self.s('\n')
  }

  pub fn space(self) -> Self {
    self.s(' ')
  }

  pub fn spaces(self, count: usize) -> Self {
    self.s(" ".repeat(count))
  }

  pub fn dot(self) -> Self {
    self.s('.')
  }

  pub fn colon(self) -> Self {
    self.s(':')
  }

  pub fn slash(self) -> Self {
    self.s('/')
  }

  pub fn dots(self) -> Self {
    self.s("...")
  }

  pub fn plural<T: Display>(mut self, s: T, number: usize) -> Self {
    let _ = if number == 1 {
      write!(&mut self.content, "{}", s)
    } else {
      write!(&mut self.content, "{}s", s)
    };
    self
  }

  pub fn black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.black());
    self
  }

  pub fn red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.red());
    self
  }

  pub fn green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.green());
    self
  }

  pub fn yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.yellow());
    self
  }

  pub fn blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.blue());
    self
  }

  pub fn magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.magenta());
    self
  }

  pub fn cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.cyan());
    self
  }

  pub fn white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.white());
    self
  }

  pub fn bg_black(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_black());
    self
  }

  pub fn bg_red(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_red());
    self
  }

  pub fn bg_green(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_green());
    self
  }

  pub fn bg_yellow(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_yellow());
    self
  }

  pub fn bg_blue(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_blue());
    self
  }

  pub fn bg_magenta(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_magenta());
    self
  }

  pub fn bg_cyan(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_cyan());
    self
  }

  pub fn bg_white(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_white());
    self
  }

  pub fn color(mut self, value: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color(value));
    self
  }

  pub fn bg_color(mut self, value: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color(value));
    self
  }

  pub fn color_256(mut self, value: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.color_256(value));
    self
  }

  pub fn bg_color_256(mut self, value: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_color_256(value));
    self
  }

  pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.rgb(r, g, b));
    self
  }

  pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bg_rgb(r, g, b));
    self
  }

  pub fn bold(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.bold());
    self
  }

  pub fn italic(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.italic());
    self
  }

  pub fn underline(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.underline());
    self
  }

  pub fn clear(mut self) -> Self {
    let _ = write!(&mut self.content, "{}", self.color_mode.clear());
    self
  }

  pub fn print(&self) {
    print!("{}", self.content);
  }

  pub fn cprint(&self) {
    print!("{}{}", self.content, self.color_mode.clear());
  }

  pub fn println(&self) {
    println!("{}", self.content);
  }

  pub fn cprintln(&self) {
    println!("{}{}", self.content, self.color_mode.clear());
  }
}

impl Add for Text {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let mut content = self.content;
    content.push_str(&rhs.content);
    Self {
      color_mode: self.color_mode,
      content,
    }
  }
}
