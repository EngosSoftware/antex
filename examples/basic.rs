//! # Basic example
//!
//! Coloring is active in the terminal, but is inactive when redirected to another process.
//!
//! Running this example in terminal will display colored text:
//!
//! ```shell
//! cargo run --example basic
//! ```
//!
//! Redirecting the output to another process will cancel coloring:
//!
//! ```shell
//! cargo run --example basic | cat
//! ```

use antex::{StyledText, Text};

fn main() {
  println!(
    "{}",
    Text::auto()
      .s("Foreground colors:\n")
      .black()
      .s(" 0 ")
      .red()
      .s(" 1 ")
      .green()
      .s(" 2 ")
      .yellow()
      .s(" 3 ")
      .blue()
      .s(" 4 ")
      .magenta()
      .s(" 5 ")
      .cyan()
      .s(" 6 ")
      .white()
      .s(" 7 ")
      .clear()
  );
}
