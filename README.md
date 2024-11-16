# Universal package manager for MatuushOS, Windows, macOS, Linux, ...
[![Clippy check](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml/badge.svg)](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml)

This is home to `pm`, a package manager that was firstly designed for MatuushOS, but then redesigned to be cross-platform. It uses 

## Installation

You can just clone this repository, run `cargo build --workspace --release` and copy the contents of `target/release` directory to system path.

## To do list
- [x] Make installing packages work
  - [x] Windows
  - [x] Unix like OSes (macOS, Linux, BSDs)

## `pm` options

```
[*] Package manager
[*] Usage: target/debug/pm OPTIONS [ARGUMENTS]
 |  
[*] Commands:
 |  
[*] generate [LOC]      Generate example configuration file
[*] docs                Generate documentation. Make sure you pipe it to tee or to redirect the output to Markdown file
[*] build [PKG]         Build file

```