### antex

[![crates.io][crates-badge]][crates-url]
![build][build-badge]
[![docs][docs-badge]][docs-url]
![coverage][cov-badge]
![license][license-badge]
![human][human-badge]

[crates-badge]: https://img.shields.io/crates/v/antex.svg
[crates-url]: https://crates.io/crates/antex
[build-badge]: https://github.com/EngosSoftware/antex/actions/workflows/build.yml/badge.svg
[docs-badge]: https://docs.rs/antex/badge.svg
[docs-url]: https://docs.rs/antex
[cov-badge]: https://img.shields.io/badge/coverage-0%25-21B577.svg
[license-badge]: https://img.shields.io/badge/license-MIT_OR_Apache--2.0-9370DB.svg
[human-badge]: https://img.shields.io/badge/made_by-HUMAN-F5DEB3.svg

# Styled text and tree in terminal

## Overview

**antex** is a lightweight Rust library for building styled text and hierarchical tree
structures in terminal applications.

**antex** provides simple, expressive tools for formatting text with colors and styles,
and for rendering clean, readable tree views in command-line interfaces.

**antex** is perfect for developing tools that leverage rich, colored terminal output
to improve clarity, structure, and user experience, without pulling in heavy UI frameworks.

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
                             .reset().s(' ')
                             .green().s("world")
                             .reset().s('!');
  println!("{}", greeting);
}
```

## License

Licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE) and [NOTICE](NOTICE))
- [MIT license](https://opensource.org/licenses/mit)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Any contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
