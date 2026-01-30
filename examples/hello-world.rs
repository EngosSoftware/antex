//! # Hello world example

use antex::{StyledText, Text};

fn main() {
  let greeting = Text::auto().yellow().s("Hello").clear().s(' ').green().s("world").clear().s('!');
  println!("{}", greeting);
}
