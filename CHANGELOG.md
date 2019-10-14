# Changelog

[`ssd1306`](https://crates.io/crates/ssd1306) is a no-std driver written in Rust for the popular SSD1306 monochrome OLED display.

## Unreleased

### Fixed

- `TerminalMode` now has a cursor. Newlines (`\n`) and carriage returns (`\r`) are now supported.

## 0.3.0-alpha.1

### Changed

- **(breaking)** Upgraded to Embedded Graphics 0.6.0-alpha.2

## 0.2.6

TODO: Description

### Added

- Added a changelog!
- Display power control (#86) - call `.display_on(true)` or `.display_on(false)` to turn on or off the display respectively.

### Fixed

- Doc examples are now tested in CI (#89)

### Changed

- Builder docs clarify the order of method calls (#89)
