//! # Styling is never active
//!
//! Running this example in terminal will display white text:
//!
//! ```shell
//! cargo run --example never
//! ```
//!
//! Redirecting the output to another process will also print white text:
//!
//! ```shell
//! cargo run --example never | cat
//! ```

use antex::{StyledText, Text};

fn main() {
  println!(
    "{}",
    Text::off()
      .s("Never colored:\n")
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
      .reset()
  );
}
