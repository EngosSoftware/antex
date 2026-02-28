### antex

[![crates.io][crates-badge]][crates-url]
[![coverage][cov-badge]][cov-url]  
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build macOs][build-badge-macos]
![build macOs arm64][build-badge-macos-arm64]  
[![mit-license][mit-badge]][mit-license-url]
[![apache-license][apache-badge]][apache-license-url]
[![cc][cc-badge]][cc-url]  
[![mbh][mbh-badge]][mbh-url]
[![es][es-badge]][es-url]

[crates-badge]: https://img.shields.io/crates/v/antex.svg
[crates-url]: https://crates.io/crates/antex
[cov-badge]: https://img.shields.io/badge/coverage-100%25-21b577.svg
[cov-url]: https://crates.io/crates/coverio
[build-badge-linux]: https://github.com/EngosSoftware/antex/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/antex/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/antex/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/antex/actions/workflows/build-macos-arm64.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-4169E1.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/antex/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-4169E1.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/antex/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/antex/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4169E1.svg
[cc-url]: https://github.com/EngosSoftware/antex/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by_a-HUMAN-DC143C.svg
[mbh-url]: https://github.com/DariuszDepta
[es-badge]: https://img.shields.io/badge/at-Engos_Software-32CD32.svg
[es-url]: https://engos.de
[repository-url]: https://github.com/EngosSoftware/antex

# Styled text and tree in terminal

## Overview

**antex** is a lightweight Rust library for building styled text and hierarchical tree structures in terminal applications.
**antex** provides simple, expressive tools for formatting text with colors and styles, and for rendering clean,
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
                             .reset().s(' ')
                             .green().s("world")
                             .reset().s('!');
  println!("{}", greeting);
}
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [antex][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
