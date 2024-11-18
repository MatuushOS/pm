# Universal package manager for MatuushOS
[![Clippy check](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml/badge.svg)](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml)

This is home to `pm`, a package manager that is designed for MatuushOS.

## Installation

You can just clone this repository, run `cargo build --workspace --release` and copy the contents of `target/release` directory to system path.

## To do list
- [x] Make installing packages work.
- [ ] Add more functions for configuring MatuushOS.
- [ ] Integrate with [mtinit](https://github.com/MatuushOS/mtinit).

## `pm` options

```
[*] Package manager
[*] Usage: target/debug/pm OPTIONS [ARGUMENTS].
 |  
[*] Commands:
 |  
[*] generate [LOC]      Generate example configuration file.
[*] docs                Generate documentation for the build files. 
 |                      └── Make sure you pipe it to tee or to redirect the output to Markdown file.
[*] install [PKG]       Install package.
[*] remove [PKG]        Remove package.
[*] ------------------------------
 |  For listing packages, type ls ~/.mtos/pkgs or ls /mtos/pkgs if you're root.
```