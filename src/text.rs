//! # Styled text

use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Alignment, Debug, Display};
use std::ops::{Add, AddAssign};

/// A trait representing styled text.
pub trait StyledText: Sized {
    /// Adds content to text.
    #[must_use]
    fn s<T: Display>(self, s: T) -> Self;

    /// Resets all styling and then appends formatted content.
    ///
    /// This is equivalent to calling `text.reset().s(value)`.
    #[must_use]
    fn r<T: Display>(self, s: T) -> Self {
        self.reset().s(s)
    }
    /// Resets all colors and styling flags.
    #[must_use]
    fn reset(self) -> Self;
    /// Adds repeated content to text.
    #[must_use]
    fn repeat<T: Display>(self, s: T, n: usize) -> Self;
    /// Adds `s` suffix to the content when the number is not 1.
    #[must_use]
    fn plural<T: Display>(self, s: T, n: usize) -> Self;
    /// Adds the content aligned left with specified width.
    #[must_use]
    fn align_left<T: Display>(self, s: T, width: usize) -> Self;
    /// Adds the content aligned right with specified width.
    #[must_use]
    fn align_right<T: Display>(self, s: T, width: usize) -> Self;
    /// Adds the content centered with specified width.
    #[must_use]
    fn align_center<T: Display>(self, s: T, width: usize) -> Self;
    /// Pads the content with specified character.
    #[must_use]
    fn pad<T: Display>(self, ch: char, padding: usize, s: T) -> Self;
    /// Pads the content aligned the left with specified width.
    #[must_use]
    fn pad_left<T: Display>(self, ch: char, s: T, width: usize) -> Self;
    /// Pads the content aligned right with specified width.
    #[must_use]
    fn pad_right<T: Display>(self, ch: char, s: T, width: usize) -> Self;
    /// Pads the content centered with specified width.
    #[must_use]
    fn pad_center<T: Display>(self, ch: char, s: T, width: usize) -> Self;
    /// Fills the content with the given character until the given width is reached.
    #[must_use]
    fn fill(self, ch: char, width: usize) -> Self;
    /// Adds new content based on the condition.
    #[must_use]
    fn choose<T: Display, F: Display>(self, condition: bool, when_true: T, when_false: F) -> Self;
    /// Adds new content based on the first matching condition.
    #[must_use]
    fn matches<C: IntoIterator<Item = (bool, impl Display)>>(self, conditions: C) -> Self;
    /// Adds indented content.
    #[must_use]
    fn indent<T: Display>(self, indent: usize, s: T) -> Self;
    /// Style text as bold.
    #[must_use]
    fn bold(self) -> Self;
    /// Style text as italic.
    #[must_use]
    fn italic(self) -> Self;
    /// Style text as underlined.
    #[must_use]
    fn underline(self) -> Self;
    /// Sets the foreground color to black.
    #[must_use]
    fn black(self) -> Self;
    /// Sets the foreground color to bright black.
    #[must_use]
    fn bright_black(self) -> Self;
    /// Sets the foreground color to red.
    #[must_use]
    fn red(self) -> Self;
    /// Sets the foreground color to bright red.
    #[must_use]
    fn bright_red(self) -> Self;
    /// Sets the foreground color to green.
    #[must_use]
    fn green(self) -> Self;
    /// Sets the foreground color to bright green.
    #[must_use]
    fn bright_green(self) -> Self;
    /// Sets the foreground color to yellow.
    #[must_use]
    fn yellow(self) -> Self;
    /// Sets the foreground color to bright yellow.
    #[must_use]
    fn bright_yellow(self) -> Self;
    /// Sets the foreground color to blue.
    #[must_use]
    fn blue(self) -> Self;
    /// Sets the foreground color to bright blue.
    #[must_use]
    fn bright_blue(self) -> Self;
    /// Sets the foreground color to magenta.
    #[must_use]
    fn magenta(self) -> Self;
    /// Sets the foreground color to bright magenta.
    #[must_use]
    fn bright_magenta(self) -> Self;
    /// Sets the foreground color to cyan.
    #[must_use]
    fn cyan(self) -> Self;
    /// Sets the foreground color to bright cyan.
    #[must_use]
    fn bright_cyan(self) -> Self;
    /// Sets the foreground color to white.
    #[must_use]
    fn white(self) -> Self;
    /// Sets the foreground color to bright white.
    #[must_use]
    fn bright_white(self) -> Self;
    /// Sets the background color to black.
    #[must_use]
    fn bg_black(self) -> Self;
    /// Sets the background color to bright black.
    #[must_use]
    fn bg_bright_black(self) -> Self;
    /// Sets the background color to red.
    #[must_use]
    fn bg_red(self) -> Self;
    /// Sets the background color to bright red.
    #[must_use]
    fn bg_bright_red(self) -> Self;
    /// Sets the background color to green.
    #[must_use]
    fn bg_green(self) -> Self;
    /// Sets the background color to bright green.
    #[must_use]
    fn bg_bright_green(self) -> Self;
    /// Sets the background color to yellow.
    #[must_use]
    fn bg_yellow(self) -> Self;
    /// Sets the background color to bright yellow.
    #[must_use]
    fn bg_bright_yellow(self) -> Self;
    /// Sets the background color to blue.
    #[must_use]
    fn bg_blue(self) -> Self;
    /// Sets the background color to bright blue.
    #[must_use]
    fn bg_bright_blue(self) -> Self;
    /// Sets the background color to magenta.
    #[must_use]
    fn bg_magenta(self) -> Self;
    /// Sets the background color to bright magenta.
    #[must_use]
    fn bg_bright_magenta(self) -> Self;
    /// Sets the background color to cyan.
    #[must_use]
    fn bg_cyan(self) -> Self;
    /// Sets the background color to bright cyan.
    #[must_use]
    fn bg_bright_cyan(self) -> Self;
    /// Sets the background color to white.
    #[must_use]
    fn bg_white(self) -> Self;
    /// Sets the background color to bright white.
    #[must_use]
    fn bg_bright_white(self) -> Self;
    /// Sets the color by color enumeration.
    #[must_use]
    fn color(self, c: Color) -> Self;
    /// Sets the bright color by color enumeration.
    #[must_use]
    fn bright_color(self, c: Color) -> Self;
    /// Sets the background color by color enumeration.
    #[must_use]
    fn bg_color(self, c: Color) -> Self;
    /// Sets the background bright color by color enumeration.
    #[must_use]
    fn bg_bright_color(self, c: Color) -> Self;
    /// Sets the color by color index.
    #[must_use]
    fn color_8(self, c: u8) -> Self;
    /// Sets the bright color by color index.
    #[must_use]
    fn bright_color_8(self, c: u8) -> Self;
    /// Sets the background color by color index.
    #[must_use]
    fn bg_color_8(self, c: u8) -> Self;
    /// Sets the background bright color by color index.
    #[must_use]
    fn bg_bright_color_8(self, c: u8) -> Self;
    #[must_use]
    fn color_256(self, c: u8) -> Self;
    #[must_use]
    fn bg_color_256(self, c: u8) -> Self;
    #[must_use]
    fn color_rgb(self, c: RgbColor) -> Self;
    #[must_use]
    fn bg_color_rgb(self, c: RgbColor) -> Self;
}

#[derive(Debug, Clone)]
enum Sequence {
    Char(char),
    Control(Cow<'static, str>),
}

impl Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(ch) => write!(f, "{ch}"),
            Self::Control(s) => write!(f, "{s}"),
        }
    }
}

impl Sequence {
    const fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }
}

#[derive(Debug, Default, Clone)]
struct Content(Vec<Sequence>);

impl Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for sequence in &self.0 {
            write!(f, "{sequence}")?;
        }
        Ok(())
    }
}

impl Content {
    /// Returns an iterator over characters.
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().filter_map(|sequence| match sequence {
            Sequence::Char(c) => Some(*c),
            Sequence::Control(_) => None,
        })
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, max: usize) -> fmt::Result {
        let mut count = 0;
        for sequence in &self.0 {
            write!(f, "{sequence}")?;
            if sequence.is_char() {
                count += 1;
            }
            if count == max {
                break;
            }
        }
        Ok(())
    }

    fn chr(&mut self, ch: char) {
        self.0.push(Sequence::Char(ch));
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

    fn ctrl(&mut self, s: Cow<'static, str>) {
        self.0.push(Sequence::Control(s));
    }

    fn append(&mut self, s: &Self) {
        for sequence in &s.0 {
            self.0.push(sequence.clone());
        }
    }

    fn pad(&mut self, ch: char, n: usize) {
        for _ in 0..n {
            self.chr(ch);
        }
    }

    fn fill(&mut self, ch: char, length: usize) {
        for _ in 0..length.saturating_sub(self.chars().count()) {
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

impl AsRef<Self> for Text {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            let fill = width.saturating_sub(self.content.chars().count());
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
    #[must_use]
    pub fn new(cm: ColorMode) -> Self {
        Self {
            cm,
            content: Content::default(),
        }
    }

    #[must_use]
    pub fn auto() -> Self {
        Self {
            cm: ColorMode::default(),
            content: Content::default(),
        }
    }

    #[must_use]
    pub fn on() -> Self {
        Self {
            cm: ColorMode::On,
            content: Content::default(),
        }
    }

    #[must_use]
    pub fn off() -> Self {
        Self {
            cm: ColorMode::Off,
            content: Content::default(),
        }
    }

    /// Returns an iterator over characters in text.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.content.chars()
    }

    /// Returns the number of characters in text.
    #[must_use]
    pub fn count(&self) -> usize {
        self.chars().count()
    }

    /// Returns text containing only characters,
    /// all control sequences (if present) are omitted.
    ///
    /// It is an equivalent of calling `chars().collect::<String>()`.
    #[must_use]
    pub fn characters(&self) -> String {
        self.content.chars().collect()
    }
}

impl StyledText for Text {
    fn s<T: Display>(mut self, s: T) -> Self {
        self.content.str(&format!("{s}"));
        self
    }

    fn reset(mut self) -> Self {
        self.content.ctrl(self.cm.reset().into());
        self
    }

    fn repeat<T: Display>(self, s: T, n: usize) -> Self {
        self.s(s.to_string().repeat(n))
    }

    fn plural<T: Display>(self, s: T, n: usize) -> Self {
        if n == 1 {
            self.s(s)
        } else {
            self.s(format!("{s}s"))
        }
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
        self.content.str(&format!("{s}"));
        self
    }

    fn pad_left<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        self.content
            .pad(ch, width.saturating_sub(buffer.chars().count()));
        self.content.str_n(&buffer, width);
        self
    }

    fn pad_right<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        self.content.str_n(&buffer, width);
        self.content
            .pad(ch, width.saturating_sub(buffer.chars().count()));
        self
    }

    fn pad_center<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        let fill = width.saturating_sub(buffer.chars().count());
        let left_fill = fill / 2;
        self.content.pad(ch, left_fill);
        self.content.str_n(&buffer, width);
        self.content.pad(ch, fill - left_fill);
        self
    }

    fn fill(mut self, ch: char, width: usize) -> Self {
        self.content.fill(ch, width);
        self
    }

    fn choose<T: Display, F: Display>(self, condition: bool, when_true: T, when_false: F) -> Self {
        self.s(if condition {
            when_true.to_string()
        } else {
            when_false.to_string()
        })
    }

    fn matches<C: IntoIterator<Item = (bool, impl Display)>>(self, conditions: C) -> Self {
        for (condition, content) in conditions {
            if condition {
                return self.s(content);
            }
        }
        self
    }

    fn indent<T: Display>(self, indent: usize, s: T) -> Self {
        self.pad(' ', indent, s)
    }

    fn bold(mut self) -> Self {
        self.content.ctrl(self.cm.bold().into());
        self
    }

    fn italic(mut self) -> Self {
        self.content.ctrl(self.cm.italic().into());
        self
    }

    fn underline(mut self) -> Self {
        self.content.ctrl(self.cm.underline().into());
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
        self.content.ctrl(self.cm.color_256(c).into());
        self
    }

    fn bg_color_256(mut self, c: u8) -> Self {
        self.content.ctrl(self.cm.bg_color_256(c).into());
        self
    }

    fn color_rgb(mut self, c: RgbColor) -> Self {
        self.content.ctrl(self.cm.color_rgb(c).into());
        self
    }

    fn bg_color_rgb(mut self, c: RgbColor) -> Self {
        self.content.ctrl(self.cm.bg_color_rgb(c).into());
        self
    }
}

impl AddAssign<Self> for Text {
    fn add_assign(&mut self, rhs: Self) {
        self.content.append(&rhs.content);
    }
}

impl AddAssign<&Self> for Text {
    fn add_assign(&mut self, rhs: &Self) {
        self.content.append(&rhs.content);
    }
}

impl AddAssign<&str> for Text {
    fn add_assign(&mut self, rhs: &str) {
        self.content.str(rhs);
    }
}

impl AddAssign<String> for Text {
    fn add_assign(&mut self, rhs: String) {
        self.content.str(&rhs);
    }
}

impl AddAssign<&String> for Text {
    fn add_assign(&mut self, rhs: &String) {
        self.content.str(rhs);
    }
}

impl AddAssign<char> for Text {
    fn add_assign(&mut self, rhs: char) {
        self.content.str(&rhs.to_string());
    }
}

impl Add<Self> for Text {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Add<&Self> for Text {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}

impl Add<&str> for Text {
    type Output = Self;
    fn add(mut self, rhs: &str) -> Self {
        self += rhs;
        self
    }
}

impl Add<String> for Text {
    type Output = Self;
    fn add(mut self, rhs: String) -> Self {
        self += rhs;
        self
    }
}

impl Add<&String> for Text {
    type Output = Self;
    fn add(mut self, rhs: &String) -> Self {
        self += rhs;
        self
    }
}

impl Add<char> for Text {
    type Output = Self;
    fn add(mut self, rhs: char) -> Self {
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
        Text {
            cm: rhs.cm,
            content,
        }
    }
}

impl Add<Text> for String {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        self.add(&rhs)
    }
}

impl Add<&Text> for String {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        let mut content = Content::default();
        content.str(&self);
        content.append(&rhs.content);
        Text {
            cm: rhs.cm,
            content,
        }
    }
}

impl Add<Text> for &String {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        self.add(&rhs)
    }
}

impl Add<&Text> for &String {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        self.as_str().add(rhs)
    }
}

impl Add<Text> for char {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        self.add(&rhs)
    }
}

impl Add<&Text> for char {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        let mut content = Content::default();
        content.chr(self);
        content.append(&rhs.content);
        Text {
            cm: rhs.cm,
            content,
        }
    }
}

#[must_use]
pub fn auto() -> Text {
    Text::auto()
}

#[must_use]
pub fn always() -> Text {
    Text::on()
}

#[must_use]
pub fn never() -> Text {
    Text::off()
}
