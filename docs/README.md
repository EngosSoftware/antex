# Styled text and tree in terminal

## Overview

**antex** is a lightweight Rust library for building styled text and hierarchical tree structures in terminal applications.
It provides simple, expressive tools for formatting text with colors and styles, and for rendering clean,
readable tree views in command-line interfaces.
**antex** is perfect for developer tools that leverage rich, colored terminal output to improve clarity,
structure, and user experience, without pulling in heavy UI frameworks.

Key features:
- styled terminal text with color and formatting support,
- colored, structured tree views for hierarchical data,
- clean and visually appealing console output,
- developer-friendly APIs for building styled output programmatically,
- lightweight and idiomatic Rust design.

## Usage

```Rust
use antex::{StyledText, Text};

fn main() {
  let greeting = Text::auto().yellow().s("Hello")
                             .normal().s(' ')
                             .green().s("world")
                             .normal().s('!');
  println!("{}", greeting);
}
```
