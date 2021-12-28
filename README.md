# OpString Library
A small library for Rust that wraps around normal strings to provide a more intuitive interface for characters.

## Installation
To add the library to your project, add it as a dependency to the cargo build system. Simply add the following line to `[dependencies]` to let cargo auto-pull the repo:
```
opstring = { git = "https://github.com/Lut99/opstring", branch="main" }
```
You can also refer a specific version by defining a tag:
```
opstring = { git = "https://github.com/Lut99/opstring", tag="v1.0.0" }
```
All tags can be found in the [tags](https://github.com/Lut99/opstring/tags) page.

## Testing
The OpString library has build-in unit tests. Run them for the library by running:
```
cargo test
```
either in the library's root folder or in the root folder of the project including this library.

## Usage
For a complete overview of what the library has to offer, check the [wiki](https://github.com/Lut99/opstring/wiki) page.

## Dependencies
The OpString library only depends on the [`unicode-segmentation`](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/) package.

## Contribution
Do you have a suggestion, bugfix or something you don't like? Let it know by creating an issues in the [issues](https://github.com/Lut99/opstring/issues) page, and we'll look into it as soon as we can.

You're also free to create a pull request yourself, which we will review as soon as we can.
