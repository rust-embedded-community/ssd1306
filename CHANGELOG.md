# Changelog

[`ssd1306`](https://crates.io/crates/ssd1306) is a no-std driver written in Rust for the popular SSD1306 monochrome OLED display.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0-alpha.3] - 2020-02-03

### Changed

- [#97](https://github.com/jamwaffles/ssd1306/pull/97) Use the new `Triangle` primitive from Embedded Graphics 0.6.0-alpha.2 in the three SSD1306 `graphics*.rs` examples
- [#103](https://github.com/jamwaffles/ssd1306/pull/103) Pin embedded-graphics version to 0.6.0-alpha.2

## [0.3.0-alpha.2]

### Fixed

- [#80](https://github.com/jamwaffles/ssd1306/pull/80) `TerminalMode` now has a cursor. Newlines (`\n`) and carriage returns (`\r`) are now supported.

### Changed

- [#94](https://github.com/jamwaffles/ssd1306/pull/94) Programs that only change some parts of the display should now run much faster. The driver keeps track of changed pixels and only sends a bounding box of updated pixels instead of updating the entire display every time.

## [0.3.0-alpha.1]

### Changed

- **(breaking)** Upgraded to Embedded Graphics 0.6.0-alpha.2

## 0.2.6

### Added

- Added a changelog!
- Display power control (#86) - call `.display_on(true)` or `.display_on(false)` to turn on or off the display respectively.

### Fixed

- Doc examples are now tested in CI (#89)

### Changed

- Builder docs clarify the order of method calls (#89)

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.3...HEAD

[0.3.0-alpha.3]: https://github.com/jamwaffles/ssd1306/compare/linuxcnc-hal-sys-v0.3.0-alpha.2...v0.3.0-alpha.3
[0.3.0-alpha.2]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.1...v0.3.0-alpha.2
[0.3.0-alpha.1]: https://github.com/jamwaffles/ssd1306/compare/0.2.5...v0.3.0-alpha.1
