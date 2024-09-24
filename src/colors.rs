//! # Terminal color sequences

/// Color mode to switch terminal colouring `ON` or `OFF`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorMode {
    /// Switch colouring **on**.
    On,
    /// Switch colouring **off**.
    Off,
}

impl From<String> for ColorMode {
    /// Converts a string into [ColorMode].
    fn from(value: String) -> Self {
        match value.to_lowercase().trim() {
            "never" => Self::Off,
            "always" => Self::On,
            _ => {
                if atty::is(atty::Stream::Stdout) {
                    Self::On
                } else {
                    Self::Off
                }
            }
        }
    }
}

/// Color palette.
#[derive(Debug, Clone)]
pub struct ColorPalette {
    color_mode: ColorMode,
}

impl From<ColorMode> for ColorPalette {
    /// Creates a [ColorPalette] from [ColorMode].
    fn from(color_mode: ColorMode) -> Self {
        Self { color_mode }
    }
}

impl ColorPalette {
    pub fn black(&self) -> String {
        self.color_8(0)
    }

    pub fn red(&self) -> String {
        self.color_8(1)
    }

    pub fn green(&self) -> String {
        self.color_8(2)
    }

    pub fn yellow(&self) -> String {
        self.color_8(3)
    }

    pub fn blue(&self) -> String {
        self.color_8(4)
    }

    pub fn magenta(&self) -> String {
        self.color_8(5)
    }

    pub fn cyan(&self) -> String {
        self.color_8(6)
    }

    pub fn white(&self) -> String {
        self.color_8(7)
    }

    pub fn bg_black(&self) -> String {
        self.bg_color_8(0)
    }

    pub fn bg_red(&self) -> String {
        self.bg_color_8(1)
    }

    pub fn bg_green(&self) -> String {
        self.bg_color_8(2)
    }

    pub fn bg_yellow(&self) -> String {
        self.bg_color_8(3)
    }

    pub fn bg_blue(&self) -> String {
        self.bg_color_8(4)
    }

    pub fn bg_magenta(&self) -> String {
        self.bg_color_8(5)
    }

    pub fn bg_cyan(&self) -> String {
        self.bg_color_8(6)
    }

    pub fn bg_white(&self) -> String {
        self.bg_color_8(7)
    }

    pub fn color(&self, value: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[38;5;{}m", value),
            _ => "".to_string(),
        }
    }

    pub fn bg_color(&self, value: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[48;5;{}m", value),
            _ => "".to_string(),
        }
    }

    pub fn rgb(&self, r: u8, g: u8, b: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[38;2;{};{};{}m", r, g, b),
            _ => "".to_string(),
        }
    }

    pub fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[48;2;{};{};{}m", r, g, b),
            _ => "".to_string(),
        }
    }

    pub fn bold(&self) -> &str {
        match self.color_mode {
            ColorMode::On => "\u{1b}[1m",
            _ => "",
        }
    }

    pub fn clear(&self) -> &str {
        match self.color_mode {
            ColorMode::On => "\u{1b}[0m",
            _ => "",
        }
    }

    fn color_8(&self, value: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[{}m", 30 + value.clamp(0, 7)),
            _ => "".to_string(),
        }
    }

    fn bg_color_8(&self, value: u8) -> String {
        match self.color_mode {
            ColorMode::On => format!("\u{1b}[{}m", 40 + value.clamp(0, 7)),
            _ => "".to_string(),
        }
    }
}
