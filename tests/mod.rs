mod test_color_mode;
mod test_text;
mod test_tree;

macro_rules! on {
  () => {
    Text::new(ColorMode::On)
  };
}

use on;

macro_rules! off {
  () => {
    Text::new(ColorMode::Off)
  };
}

use off;

fn cm<'a>() -> &'a str {
  use std::io::IsTerminal;
  if std::io::stdout().is_terminal() {
    "On"
  } else {
    "Off"
  }
}
