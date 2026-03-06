use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use std::fmt;
use std::fmt::{Alignment, Debug, Display};
use std::ops::{Add, AddAssign};

/// A trait representing styled text.
pub trait StyledText: Sized {
  /// Adds content to text.
  fn s<T: Display>(self, s: T) -> Self;

  /// Resets all styling and then appends formatted content.
  ///
  /// This is equivalent to calling `text.reset().s(value)`.
  fn r<T: Display>(self, s: T) -> Self {
    self.reset().s(s)
  }
  /// Resets all colors and styling flags.
  fn reset(self) -> Self;
  /// Adds repeated content to text.
  fn repeat<T: Display>(self, s: T, n: usize) -> Self;
  /// Adds `s` suffix to the content when the number is not 1.
  fn plural<T: Display>(self, s: T, n: usize) -> Self;
  /// Adds the content aligned left with specified width.
  fn align_left<T: Display>(self, s: T, width: usize) -> Self;
  /// Adds the content aligned right with specified width.
  fn align_right<T: Display>(self, s: T, width: usize) -> Self;
  /// Adds the content centered with specified width.
  fn align_center<T: Display>(self, s: T, width: usize) -> Self;
  /// Pads the content with specified character.
  fn pad<T: Display>(self, ch: char, padding: usize, s: T) -> Self;
  /// Pads the content aligned the left with specified width.
  fn pad_left<T: Display>(self, ch: char, s: T, width: usize) -> Self;
  /// Pads the content aligned right with specified width.
  fn pad_right<T: Display>(self, ch: char, s: T, width: usize) -> Self;
  /// Pads the content centered with specified width.
  fn pad_center<T: Display>(self, ch: char, s: T, width: usize) -> Self;
  /// Adds new content based on the condition.
  fn choose<T: Display>(self, condition: bool, when_true: T, when_false: T) -> Self;
  /// Adds indented content.
  fn indent<T: Display>(self, indent: usize, s: T) -> Self;
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

#[derive(Debug, Clone)]
enum Sequence {
  Char(char),
  Control(String),
}

impl Display for Sequence {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Sequence::Char(ch) => write!(f, "{}", ch),
      Sequence::Control(s) => write!(f, "{}", s),
    }
  }
}

impl Sequence {
  fn is_char(&self) -> bool {
    matches!(self, Sequence::Char(_))
  }
}

#[derive(Debug, Default, Clone)]
struct Content(Vec<Sequence>);

impl Display for Content {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for sequence in &self.0 {
      write!(f, "{}", sequence)?;
    }
    Ok(())
  }
}

impl Content {
  fn count(&self) -> usize {
    self.0.iter().filter(|sequence| sequence.is_char()).count()
  }

  fn write(&self, f: &mut fmt::Formatter<'_>, max: usize) -> fmt::Result {
    let mut count = 0;
    for sequence in &self.0 {
      write!(f, "{}", sequence)?;
      if sequence.is_char() {
        count += 1;
      }
      if count == max {
        break;
      }
    }
    Ok(())
  }

  fn str(&mut self, s: &str) {
    for ch in s.chars() {
      self.0.push(Sequence::Char(ch));
    }
  }

  fn str_n(&mut self, s: &str, n: usize) {
    for ch in s.chars().take(n) {
      self.0.push(Sequence::Char(ch));
    }
  }

  fn ctrl(&mut self, s: impl ToString) {
    self.0.push(Sequence::Control(s.to_string()));
  }

  fn append(&mut self, s: &Content) {
    for sequence in &s.0 {
      self.0.push(sequence.clone());
    }
  }

  fn pad(&mut self, ch: char, n: usize) {
    for _ in 0..n {
      self.0.push(Sequence::Char(ch));
    }
  }
}

/// Styled text.
#[derive(Debug, Clone)]
pub struct Text {
  /// Color mode.
  cm: ColorMode,
  /// Text content.
  content: Content,
}

impl Display for Text {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(width) = f.width() {
      let fill = width.saturating_sub(self.content.count());
      let ch = f.fill().to_string();
      if let Some(align) = f.align() {
        match align {
          Alignment::Left => {
            self.content.write(f, width)?;
            write!(f, "{}", ch.repeat(fill))?;
          }
          Alignment::Right => {
            write!(f, "{}", ch.repeat(fill))?;
            self.content.write(f, width)?;
          }
          Alignment::Center => {
            let left_fill = fill / 2;
            write!(f, "{}", ch.repeat(left_fill))?;
            self.content.write(f, width)?;
            write!(f, "{}", ch.repeat(fill - left_fill))?;
          }
        }
      } else {
        self.content.write(f, width)?;
        write!(f, "{}", ch.repeat(fill))?;
      }
    } else {
      write!(f, "{}", self.content)?;
    }
    Ok(())
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
    Self { cm, content: Default::default() }
  }

  pub fn auto() -> Self {
    Self {
      cm: ColorMode::default(),
      content: Default::default(),
    }
  }

  pub fn on() -> Self {
    Self {
      cm: ColorMode::On,
      content: Default::default(),
    }
  }

  pub fn off() -> Self {
    Self {
      cm: ColorMode::Off,
      content: Default::default(),
    }
  }
}

impl StyledText for Text {
  fn s<T: Display>(mut self, s: T) -> Self {
    self.content.str(&format!("{}", s));
    self
  }

  fn reset(mut self) -> Self {
    self.content.ctrl(self.cm.reset());
    self
  }

  fn repeat<T: Display>(self, s: T, n: usize) -> Self {
    self.s(s.to_string().repeat(n))
  }

  fn plural<T: Display>(self, s: T, n: usize) -> Self {
    if n == 1 { self.s(s) } else { self.s(format!("{}s", s)) }
  }

  fn align_left<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_right(' ', s, width)
  }

  fn align_right<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_left(' ', s, width)
  }

  fn align_center<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_center(' ', s, width)
  }

  fn pad<T: Display>(mut self, ch: char, padding: usize, s: T) -> Self {
    self.content.pad(ch, padding);
    self.content.str(&format!("{}", s));
    self
  }

  fn pad_left<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    let buffer = format!("{}", s);
    self.content.pad(ch, width.saturating_sub(buffer.chars().count()));
    self.content.str_n(&buffer, width);
    self
  }

  fn pad_right<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    let buffer = format!("{}", s);
    self.content.str_n(&buffer, width);
    self.content.pad(ch, width.saturating_sub(buffer.chars().count()));
    self
  }

  fn pad_center<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    let buffer = format!("{}", s);
    let fill = width.saturating_sub(buffer.chars().count());
    let left_fill = fill / 2;
    self.content.pad(ch, left_fill);
    self.content.str_n(&buffer, width);
    self.content.pad(ch, fill - left_fill);
    self
  }

  fn choose<T: Display>(self, condition: bool, when_true: T, when_false: T) -> Self {
    self.s(if condition { when_true } else { when_false })
  }

  fn indent<T: Display>(self, indent: usize, s: T) -> Self {
    self.pad(' ', indent, s)
  }

  fn bold(mut self) -> Self {
    self.content.ctrl(self.cm.bold());
    self
  }

  fn italic(mut self) -> Self {
    self.content.ctrl(self.cm.italic());
    self
  }

  fn underline(mut self) -> Self {
    self.content.ctrl(self.cm.underline());
    self
  }

  fn black(mut self) -> Self {
    self.content.ctrl(self.cm.black(false));
    self
  }

  fn bright_black(mut self) -> Self {
    self.content.ctrl(self.cm.black(true));
    self
  }

  fn red(mut self) -> Self {
    self.content.ctrl(self.cm.red(false));
    self
  }

  fn bright_red(mut self) -> Self {
    self.content.ctrl(self.cm.red(true));
    self
  }

  fn green(mut self) -> Self {
    self.content.ctrl(self.cm.green(false));
    self
  }

  fn bright_green(mut self) -> Self {
    self.content.ctrl(self.cm.green(true));
    self
  }

  fn yellow(mut self) -> Self {
    self.content.ctrl(self.cm.yellow(false));
    self
  }

  fn bright_yellow(mut self) -> Self {
    self.content.ctrl(self.cm.yellow(true));
    self
  }

  fn blue(mut self) -> Self {
    self.content.ctrl(self.cm.blue(false));
    self
  }

  fn bright_blue(mut self) -> Self {
    self.content.ctrl(self.cm.blue(true));
    self
  }

  fn magenta(mut self) -> Self {
    self.content.ctrl(self.cm.magenta(false));
    self
  }

  fn bright_magenta(mut self) -> Self {
    self.content.ctrl(self.cm.magenta(true));
    self
  }

  fn cyan(mut self) -> Self {
    self.content.ctrl(self.cm.cyan(false));
    self
  }

  fn bright_cyan(mut self) -> Self {
    self.content.ctrl(self.cm.cyan(true));
    self
  }

  fn white(mut self) -> Self {
    self.content.ctrl(self.cm.white(false));
    self
  }

  fn bright_white(mut self) -> Self {
    self.content.ctrl(self.cm.white(true));
    self
  }

  fn bg_black(mut self) -> Self {
    self.content.ctrl(self.cm.bg_black(false));
    self
  }

  fn bg_bright_black(mut self) -> Self {
    self.content.ctrl(self.cm.bg_black(true));
    self
  }

  fn bg_red(mut self) -> Self {
    self.content.ctrl(self.cm.bg_red(false));
    self
  }

  fn bg_bright_red(mut self) -> Self {
    self.content.ctrl(self.cm.bg_red(true));
    self
  }

  fn bg_green(mut self) -> Self {
    self.content.ctrl(self.cm.bg_green(false));
    self
  }

  fn bg_bright_green(mut self) -> Self {
    self.content.ctrl(self.cm.bg_green(true));
    self
  }

  fn bg_yellow(mut self) -> Self {
    self.content.ctrl(self.cm.bg_yellow(false));
    self
  }

  fn bg_bright_yellow(mut self) -> Self {
    self.content.ctrl(self.cm.bg_yellow(true));
    self
  }

  fn bg_blue(mut self) -> Self {
    self.content.ctrl(self.cm.bg_blue(false));
    self
  }

  fn bg_bright_blue(mut self) -> Self {
    self.content.ctrl(self.cm.bg_blue(true));
    self
  }

  fn bg_magenta(mut self) -> Self {
    self.content.ctrl(self.cm.bg_magenta(false));
    self
  }

  fn bg_bright_magenta(mut self) -> Self {
    self.content.ctrl(self.cm.bg_magenta(true));
    self
  }

  fn bg_cyan(mut self) -> Self {
    self.content.ctrl(self.cm.bg_cyan(false));
    self
  }

  fn bg_bright_cyan(mut self) -> Self {
    self.content.ctrl(self.cm.bg_cyan(true));
    self
  }

  fn bg_white(mut self) -> Self {
    self.content.ctrl(self.cm.bg_white(false));
    self
  }

  fn bg_bright_white(mut self) -> Self {
    self.content.ctrl(self.cm.bg_white(true));
    self
  }

  fn color(mut self, c: Color) -> Self {
    self.content.ctrl(self.cm.color(c, false));
    self
  }

  fn bright_color(mut self, c: Color) -> Self {
    self.content.ctrl(self.cm.color(c, true));
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    self.content.ctrl(self.cm.bg_color(c, false));
    self
  }

  fn bg_bright_color(mut self, c: Color) -> Self {
    self.content.ctrl(self.cm.bg_color(c, true));
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.color_8(c, false));
    self
  }

  fn bright_color_8(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.color_8(c, true));
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.bg_color_8(c, false));
    self
  }

  fn bg_bright_color_8(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.bg_color_8(c, true));
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.color_256(c));
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    self.content.ctrl(self.cm.bg_color_256(c));
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    self.content.ctrl(self.cm.color_rgb(c));
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    self.content.ctrl(self.cm.bg_color_rgb(c));
    self
  }
}

impl AddAssign<Text> for Text {
  fn add_assign(&mut self, rhs: Text) {
    self.content.append(&rhs.content);
  }
}

impl AddAssign<&Text> for Text {
  fn add_assign(&mut self, rhs: &Text) {
    self.content.append(&rhs.content);
  }
}

impl AddAssign<&str> for Text {
  fn add_assign(&mut self, rhs: &str) {
    self.content.str(rhs);
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
    let mut content = Content::default();
    content.str(self);
    content.append(&rhs.content);
    Text { cm: rhs.cm, content }
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
