//! # Hello world example

use antex::{StyledText, Text};

fn main() {
  let greeting = Text::auto().yellow().s("Hello").normal().s(' ').green().s("world").normal().s('!');
  println!("{}", greeting);
}
