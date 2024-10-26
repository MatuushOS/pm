# Universal package manager for MatuushOS, Windows, macOS, Linux, ...
[![.github/workflows/build.yml](https://github.com/MatuushOS/pm/actions/workflows/build.yml/badge.svg)](https://github.com/MatuushOS/pm/actions/workflows/build.yml)
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
| -------------------- | ------- | -------------- |
| Building packages    | ❎       | ✅              |
| Package installation | ✅       | ✅              |
