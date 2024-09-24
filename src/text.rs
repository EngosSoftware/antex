use crate::colors::{ColorMode, ColorPalette};
use std::fmt::Write;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Text {
    color_palette: ColorPalette,
    content: String,
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new(ColorMode::On)
    }
}

impl From<ColorMode> for Text {
    fn from(color_mode: ColorMode) -> Self {
        Self::new(color_mode)
    }
}

impl Text {
    pub fn new(color_mode: ColorMode) -> Self {
        Self {
            color_palette: color_mode.into(),
            content: String::default(),
        }
    }

    pub fn append(mut self, text: &str) -> Self {
        let _ = write!(&mut self.content, "{}", text);
        self
    }

    pub fn nl(mut self) -> Self {
        let _ = writeln!(&mut self.content);
        self
    }

    pub fn black(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.black());
        self
    }

    pub fn red(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.red());
        self
    }

    pub fn green(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.green());
        self
    }

    pub fn yellow(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.yellow());
        self
    }

    pub fn blue(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.blue());
        self
    }

    pub fn magenta(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.magenta());
        self
    }

    pub fn cyan(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.cyan());
        self
    }

    pub fn white(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.white());
        self
    }

    pub fn bg_black(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_black());
        self
    }

    pub fn bg_red(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_red());
        self
    }

    pub fn bg_green(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_green());
        self
    }

    pub fn bg_yellow(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_yellow());
        self
    }

    pub fn bg_blue(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_blue());
        self
    }

    pub fn bg_magenta(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_magenta());
        self
    }

    pub fn bg_cyan(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_cyan());
        self
    }

    pub fn bg_white(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_white());
        self
    }

    pub fn color(mut self, value: u8) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.color(value));
        self
    }

    pub fn bg_color(mut self, value: u8) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_color(value));
        self
    }

    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.rgb(r, g, b));
        self
    }

    pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bg_rgb(r, g, b));
        self
    }

    pub fn bold(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.bold());
        self
    }

    pub fn italic(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.italic());
        self
    }

    pub fn underline(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.underline());
        self
    }

    pub fn clear(mut self) -> Self {
        let _ = write!(&mut self.content, "{}", self.color_palette.clear());
        self
    }
}

impl Add for Text {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut content = self.content;
        content.push_str(&rhs.content);
        Self {
            color_palette: self.color_palette,
            content,
        }
    }
}
