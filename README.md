# Universal package manager for MatuushOS, Windows, macOS, Linux, ...
[![Clippy check](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml/badge.svg)](https://github.com/MatuushOS/pm/actions/workflows/clippy.yml)

This is home to `pm`, a package manager that was firstly designed for MatuushOS, but then redesigned to be cross-platform.

## Installation

You can just clone this repository, run `cargo build --workspace --release` and copy the contents of `target/release` directory to system path.

## To do list
- [ ] Make installing packages work
  - [x] Windows
  - [ ] Unix like OSes (macOS, Linux, BSDs)
- [ ] Write tests that cover every binary target
- [ ] Decouple traits and trait implementations from `package_manager` crate

## Feature comparison

|                      | Windows | Unix-like OSes |
|----------------------|---------|----------------|
| Building packages    | ❎       | ✅              |
| Package installation | ✅       | ✅              |

## `pm` options

```
Package manager

Usage: pm [OPTIONS]

Options:
  -i, --install <INSTALL>  Installs a package
  -r, --remove <REMOVE>    Removes a package
  -q, --query <QUERY>      Queries a package
  -c, --create <CREATE>    Creates a new configuration file
  -h, --help               Print help
```