//! # Styled text

use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use std::borrow::Cow;
use std::fmt::{Alignment, Debug, Display, Formatter, Result, Write};
use std::iter::repeat_n;
use std::ops::{Add, AddAssign};

pub mod internal {
    pub struct Token(pub(crate) ());

    pub trait Sealed {
        fn text(&mut self, t: Token) -> &mut super::Text;
    }
}

const TOKEN: internal::Token = internal::Token(());

/// A trait representing styled text.
pub trait StyledText: internal::Sealed + Sized {
    /// Appends content to text.
    #[must_use]
    fn s<T: Display>(mut self, s: T) -> Self {
        self.text(internal::Token(())).content.str(s.to_string());
        self
    }

    /// Resets colors and styling before appending content to text.
    #[must_use]
    fn r<T: Display>(mut self, s: T) -> Self {
        let text = self.text(TOKEN);
        text.content.ctrl(text.cm.reset().into());
        text.content.str(s.to_string());
        self
    }

    /// Resets colors and styling.
    #[must_use]
    fn reset(mut self) -> Self {
        let text = self.text(TOKEN);
        text.content.ctrl(text.cm.reset().into());
        self
    }

    /// Appends repeated content to text.
    #[must_use]
    fn repeat<T: Display>(mut self, s: T, n: usize) -> Self {
        self.text(TOKEN).content.str(s.to_string().repeat(n));
        self
    }

    /// Adds `s` suffix to the content when the number is not 1.
    #[must_use]
    fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
        let text = self.text(TOKEN);
        text.content.str(s.to_string());
        if n != 1 {
            text.content.str("s");
        }
        self
    }

    /// Adds the content aligned left within specified width.
    #[must_use]
    fn align_left<T: Display>(self, s: T, width: usize) -> Self {
        self.pad_right(' ', s, width)
    }

    /// Adds the content aligned right within specified width.
    #[must_use]
    fn align_right<T: Display>(self, s: T, width: usize) -> Self {
        self.pad_left(' ', s, width)
    }

    /// Adds the content centered within specified width.
    #[must_use]
    fn align_center<T: Display>(self, s: T, width: usize) -> Self {
        self.pad_center(' ', s, width)
    }

    /// Pads the content with specified character.
    #[must_use]
    fn pad<T: Display>(mut self, ch: char, padding: usize, s: T) -> Self {
        let text = self.text(TOKEN);
        text.content.pad(ch, padding);
        text.content.str(s.to_string());
        self
    }

    /// Pads the content aligned to the left within specified width.
    #[must_use]
    fn pad_left<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        let pad_width = width.saturating_sub(buffer.chars().count());
        let text = self.text(TOKEN);
        text.content.pad(ch, pad_width);
        text.content.strn(&buffer, width);
        self
    }

    /// Pads the content aligned to the right within specified width.
    #[must_use]
    fn pad_right<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        let pad_width = width.saturating_sub(buffer.chars().count());
        let text = self.text(TOKEN);
        text.content.strn(&buffer, width);
        text.content.pad(ch, pad_width);
        self
    }

    /// Pads the content centered within specified width.
    #[must_use]
    fn pad_center<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
        let buffer = format!("{s}");
        let pad_width = width.saturating_sub(buffer.chars().count());
        let left_pad_width = pad_width / 2;
        let text = self.text(TOKEN);
        text.content.pad(ch, left_pad_width);
        text.content.strn(&buffer, width);
        text.content.pad(ch, pad_width - left_pad_width);
        self
    }

    /// Fills the content with the given character until the given width is reached.
    #[must_use]
    fn fill(mut self, ch: char, width: usize) -> Self {
        self.text(TOKEN).content.fill(ch, width);
        self
    }

    /// Adds new content based on the condition.
    #[must_use]
    fn choose<T: Display, F: Display>(self, condition: bool, when_true: T, when_false: F) -> Self {
        if condition {
            self.s(when_true)
        } else {
            self.s(when_false)
        }
    }

    /// Adds new content based on the first matching condition.
    #[must_use]
    fn matches<C: IntoIterator<Item = (bool, impl Display)>>(self, conditions: C) -> Self {
        for (condition, content) in conditions {
            if condition {
                return self.s(content);
            }
        }
        self
    }

    /// Adds indented content.
    #[must_use]
    fn indent<T: Display>(self, indent: usize, s: T) -> Self {
        self.pad(' ', indent, s)
    }

    /// Style text as bold.
    #[must_use]
    fn bold(self) -> Self {
        ctrl(self, |cm| cm.bold().into())
    }

    /// Style text as italic.
    #[must_use]
    fn italic(self) -> Self {
        ctrl(self, |cm| cm.italic().into())
    }

    /// Style text as underlined.
    #[must_use]
    fn underline(self) -> Self {
        ctrl(self, |cm| cm.underline().into())
    }

    /// Sets the foreground color to black.
    #[must_use]
    fn black(self) -> Self {
        ctrl(self, |cm| cm.black(false))
    }

    /// Sets the foreground color to bright black.
    #[must_use]
    fn bright_black(self) -> Self {
        ctrl(self, |cm| cm.black(true))
    }

    /// Sets the foreground color to red.
    #[must_use]
    fn red(self) -> Self {
        ctrl(self, |cm| cm.red(false))
    }

    /// Sets the foreground color to bright red.
    #[must_use]
    fn bright_red(self) -> Self {
        ctrl(self, |cm| cm.red(true))
    }

    /// Sets the foreground color to green.
    #[must_use]
    fn green(self) -> Self {
        ctrl(self, |cm| cm.green(false))
    }

    /// Sets the foreground color to bright green.
    #[must_use]
    fn bright_green(self) -> Self {
        ctrl(self, |cm| cm.green(true))
    }

    /// Sets the foreground color to yellow.
    #[must_use]
    fn yellow(self) -> Self {
        ctrl(self, |cm| cm.yellow(false))
    }

    /// Sets the foreground color to bright yellow.
    #[must_use]
    fn bright_yellow(self) -> Self {
        ctrl(self, |cm| cm.yellow(true))
    }

    /// Sets the foreground color to blue.
    #[must_use]
    fn blue(self) -> Self {
        ctrl(self, |cm| cm.blue(false))
    }

    /// Sets the foreground color to bright blue.
    #[must_use]
    fn bright_blue(self) -> Self {
        ctrl(self, |cm| cm.blue(true))
    }

    /// Sets the foreground color to magenta.
    #[must_use]
    fn magenta(self) -> Self {
        ctrl(self, |cm| cm.magenta(false))
    }

    /// Sets the foreground color to bright magenta.
    #[must_use]
    fn bright_magenta(self) -> Self {
        ctrl(self, |cm| cm.magenta(true))
    }

    /// Sets the foreground color to cyan.
    #[must_use]
    fn cyan(self) -> Self {
        ctrl(self, |cm| cm.cyan(false))
    }

    /// Sets the foreground color to bright cyan.
    #[must_use]
    fn bright_cyan(self) -> Self {
        ctrl(self, |cm| cm.cyan(true))
    }

    /// Sets the foreground color to white.
    #[must_use]
    fn white(self) -> Self {
        ctrl(self, |cm| cm.white(false))
    }

    /// Sets the foreground color to bright white.
    #[must_use]
    fn bright_white(self) -> Self {
        ctrl(self, |cm| cm.white(true))
    }

    /// Sets the background color to black.
    #[must_use]
    fn bg_black(self) -> Self {
        ctrl(self, |cm| cm.bg_black(false))
    }

    /// Sets the background color to bright black.
    #[must_use]
    fn bg_bright_black(self) -> Self {
        ctrl(self, |cm| cm.bg_black(true))
    }

    /// Sets the background color to red.
    #[must_use]
    fn bg_red(self) -> Self {
        ctrl(self, |cm| cm.bg_red(false))
    }

    /// Sets the background color to bright red.
    #[must_use]
    fn bg_bright_red(self) -> Self {
        ctrl(self, |cm| cm.bg_red(true))
    }

    /// Sets the background color to green.
    #[must_use]
    fn bg_green(self) -> Self {
        ctrl(self, |cm| cm.bg_green(false))
    }

    /// Sets the background color to bright green.
    #[must_use]
    fn bg_bright_green(self) -> Self {
        ctrl(self, |cm| cm.bg_green(true))
    }

    /// Sets the background color to yellow.
    #[must_use]
    fn bg_yellow(self) -> Self {
        ctrl(self, |cm| cm.bg_yellow(false))
    }

    /// Sets the background color to bright yellow.
    #[must_use]
    fn bg_bright_yellow(self) -> Self {
        ctrl(self, |cm| cm.bg_yellow(true))
    }

    /// Sets the background color to blue.
    #[must_use]
    fn bg_blue(self) -> Self {
        ctrl(self, |cm| cm.bg_blue(false))
    }

    /// Sets the background color to bright blue.
    #[must_use]
    fn bg_bright_blue(self) -> Self {
        ctrl(self, |cm| cm.bg_blue(true))
    }

    /// Sets the background color to magenta.
    #[must_use]
    fn bg_magenta(self) -> Self {
        ctrl(self, |cm| cm.bg_magenta(false))
    }

    /// Sets the background color to bright magenta.
    #[must_use]
    fn bg_bright_magenta(self) -> Self {
        ctrl(self, |cm| cm.bg_magenta(true))
    }

    /// Sets the background color to cyan.
    #[must_use]
    fn bg_cyan(self) -> Self {
        ctrl(self, |cm| cm.bg_cyan(false))
    }

    /// Sets the background color to bright cyan.
    #[must_use]
    fn bg_bright_cyan(self) -> Self {
        ctrl(self, |cm| cm.bg_cyan(true))
    }

    /// Sets the background color to white.
    #[must_use]
    fn bg_white(self) -> Self {
        ctrl(self, |cm| cm.bg_white(false))
    }

    /// Sets the background color to bright white.
    #[must_use]
    fn bg_bright_white(self) -> Self {
        ctrl(self, |cm| cm.bg_white(true))
    }

    /// Sets the color by color enumeration.
    #[must_use]
    fn color(self, c: Color) -> Self {
        ctrl(self, |cm| cm.color(c, false))
    }

    /// Sets the bright color by color enumeration.
    #[must_use]
    fn bright_color(self, c: Color) -> Self {
        ctrl(self, |cm| cm.color(c, true))
    }

    /// Sets the background color by color enumeration.
    #[must_use]
    fn bg_color(self, c: Color) -> Self {
        ctrl(self, |cm| cm.bg_color(c, false))
    }

    /// Sets the background bright color by color enumeration.
    #[must_use]
    fn bg_bright_color(self, c: Color) -> Self {
        ctrl(self, |cm| cm.bg_color(c, true))
    }

    /// Sets the color by color index.
    #[must_use]
    fn color_8(self, c: u8) -> Self {
        ctrl(self, |cm| cm.color_8(c, false))
    }

    /// Sets the bright color by color index.
    #[must_use]
    fn bright_color_8(self, c: u8) -> Self {
        ctrl(self, |cm| cm.color_8(c, true))
    }

    /// Sets the background color by color index.
    #[must_use]
    fn bg_color_8(self, c: u8) -> Self {
        ctrl(self, |cm| cm.bg_color_8(c, false))
    }

    /// Sets the background bright color by color index.
    #[must_use]
    fn bg_bright_color_8(self, c: u8) -> Self {
        ctrl(self, |cm| cm.bg_color_8(c, true))
    }

    /// Sets the color using 8-bit color value.
    #[must_use]
    fn color_256(self, c: u8) -> Self {
        ctrl(self, |cm| cm.color_256(c).into())
    }

    /// Sets the background color using 8-bit color value.
    #[must_use]
    fn bg_color_256(self, c: u8) -> Self {
        ctrl(self, |cm| cm.bg_color_256(c).into())
    }

    /// Sets the color using RGB value.
    #[must_use]
    fn color_rgb(self, c: RgbColor) -> Self {
        ctrl(self, |cm| cm.color_rgb(c).into())
    }

    /// Sets the background color using RGB value.
    #[must_use]
    fn bg_color_rgb(self, c: RgbColor) -> Self {
        ctrl(self, |cm| cm.bg_color_rgb(c).into())
    }
}

#[derive(Debug, Clone)]
enum Sequence {
    Str(String),
    Ctrl(Cow<'static, str>),
}

#[derive(Debug, Default, Clone)]
struct Content(Vec<Sequence>);

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for sequence in &self.0 {
            match sequence {
                Sequence::Str(ch) => f.write_str(ch)?,
                Sequence::Ctrl(s) => f.write_str(s)?,
            }
        }
        Ok(())
    }
}

impl Content {
    /// Returns an iterator over non-control characters.
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().flat_map(|sequence| match sequence {
            Sequence::Str(c) => c.chars(),
            Sequence::Ctrl(_) => "".chars(),
        })
    }

    fn write(&self, f: &mut Formatter<'_>, max: usize) -> Result {
        let mut remaining = max;
        for sequence in &self.0 {
            match sequence {
                Sequence::Str(s) => {
                    if remaining > 0 {
                        let (count, end) = s
                            .char_indices()
                            .take(remaining)
                            .fold((0, 0), |(n, _), (i, ch)| (n + 1, i + ch.len_utf8()));
                        f.write_str(&s[..end])?;
                        remaining -= count;
                    }
                }
                Sequence::Ctrl(s) => f.write_str(s)?,
            }
        }
        Ok(())
    }

    fn str<S: AsRef<str> + Into<String>>(&mut self, str: S) {
        let str_ref = str.as_ref();
        if !str_ref.is_empty() {
            if let Some(Sequence::Str(s)) = self.0.last_mut() {
                s.push_str(str_ref);
            } else {
                self.0.push(Sequence::Str(str.into()));
            }
        }
    }

    fn strn(&mut self, str: &str, n: usize) {
        if !str.is_empty() && n > 0 {
            let take = str.chars().take(n);
            if let Some(Sequence::Str(s)) = self.0.last_mut() {
                s.extend(take);
            } else {
                self.0.push(Sequence::Str(take.collect()));
            }
        }
    }

    fn ctrl(&mut self, s: Cow<'static, str>) {
        if !s.is_empty() {
            self.0.push(Sequence::Ctrl(s));
        }
    }

    fn append(&mut self, s: Self) {
        for sequence in s.0 {
            match sequence {
                Sequence::Str(s) => self.str(s),
                Sequence::Ctrl(s) => self.ctrl(s),
            }
        }
    }

    fn pad(&mut self, ch: char, n: usize) {
        if n > 0 {
            if let Some(Sequence::Str(s)) = self.0.last_mut() {
                s.extend(repeat_n(ch, n));
            } else {
                self.0.push(Sequence::Str(repeat_n(ch, n).collect()));
            }
        }
    }

    fn fill(&mut self, ch: char, length: usize) {
        self.pad(ch, length.saturating_sub(self.chars().count()));
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let write_fill = |f: &mut Formatter, ch, count| -> Result {
            for _ in 0..count {
                f.write_char(ch)?;
            }
            Ok(())
        };
        if let Some(width) = f.width() {
            let fill_width = width.saturating_sub(self.content.chars().count());
            let fill_char = f.fill();
            if let Some(align) = f.align() {
                match align {
                    Alignment::Left => {
                        self.content.write(f, width)?;
                        write_fill(f, fill_char, fill_width)?;
                    }
                    Alignment::Right => {
                        write_fill(f, fill_char, fill_width)?;
                        self.content.write(f, width)?;
                    }
                    Alignment::Center => {
                        write_fill(f, fill_char, fill_width / 2)?;
                        self.content.write(f, width)?;
                        write_fill(f, fill_char, fill_width - fill_width / 2)?;
                    }
                }
            } else {
                self.content.write(f, width)?;
                write_fill(f, fill_char, fill_width)?;
            }
        } else {
            Display::fmt(&self.content, f)?;
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
        Self::new(ColorMode::default())
    }

    #[must_use]
    pub fn on() -> Self {
        Self::new(ColorMode::On)
    }

    #[must_use]
    pub fn off() -> Self {
        Self::new(ColorMode::Off)
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

impl internal::Sealed for Text {
    fn text(&mut self, _: internal::Token) -> &mut Text {
        self
    }
}

impl StyledText for Text {}

impl AddAssign<Self> for Text {
    fn add_assign(&mut self, rhs: Self) {
        self.content.append(rhs.content);
    }
}

impl AddAssign<&Self> for Text {
    fn add_assign(&mut self, rhs: &Self) {
        self.content.append(rhs.content.clone());
    }
}

impl AddAssign<&str> for Text {
    fn add_assign(&mut self, rhs: &str) {
        self.content.str(rhs);
    }
}

impl AddAssign<String> for Text {
    fn add_assign(&mut self, rhs: String) {
        self.content.str(rhs);
    }
}

impl AddAssign<&String> for Text {
    fn add_assign(&mut self, rhs: &String) {
        self.content.str(rhs);
    }
}

impl AddAssign<char> for Text {
    fn add_assign(&mut self, rhs: char) {
        self.content.str(rhs.to_string());
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
        prepend(self, rhs)
    }
}

impl Add<&Text> for &str {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        self.add(rhs.clone())
    }
}

impl Add<Text> for String {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        prepend(self, rhs)
    }
}

impl Add<&Text> for String {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        self.add(rhs.clone())
    }
}

impl Add<Text> for &String {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        prepend(self, rhs)
    }
}

impl Add<&Text> for &String {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        self.add(rhs.clone())
    }
}

impl Add<Text> for char {
    type Output = Text;
    fn add(self, rhs: Text) -> Text {
        let mut content = Content::default();
        content.0.push(Sequence::Str(self.to_string()));
        content.append(rhs.content);
        Text {
            cm: rhs.cm,
            content,
        }
    }
}

impl Add<&Text> for char {
    type Output = Text;
    fn add(self, rhs: &Text) -> Text {
        self.add(rhs.clone())
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

/// Utility function for prepending a [Text] with string variants.
fn prepend<S: AsRef<str> + Into<String>>(s: S, rhs: Text) -> Text {
    let mut content = Content::default();
    content.str(s);
    content.append(rhs.content);
    Text {
        cm: rhs.cm,
        content,
    }
}

fn ctrl<S: StyledText>(mut this: S, f: impl FnOnce(ColorMode) -> Cow<'static, str>) -> S {
    let text = this.text(TOKEN);
    text.content.ctrl(f(text.cm));
    this
}
