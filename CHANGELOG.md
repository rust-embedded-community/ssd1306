# Changelog

[`ssd1306`](https://crates.io/crates/ssd1306) is a no-std driver written in Rust for the popular SSD1306 monochrome OLED display.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Add support for modules with a 64x48px display size.

### Changed

- [#107](https://github.com/jamwaffles/ssd1306/pull/107) Migrate from Travis to CircleCI
- [#105](https://github.com/jamwaffles/ssd1306/pull/105) Reduce flash usage by around 400 bytes by replacing some internal `unwrap()`s with `as` coercions.
- [#106](https://github.com/jamwaffles/ssd1306/pull/106) Optimise internals by using iterators to elide bounds checks. Should also speed up `GraphicsMode` (and `embedded-graphics` operations) with a cleaned-up `set_pixel`.
- [#108](https://github.com/jamwaffles/ssd1306/pull/108) Add an example using `DisplayProperties.draw()` to send a raw buffer of random bytes to the display over I2C.

## [0.3.0-alpha.4] - 2020-02-07

### Added

- [#101](https://github.com/jamwaffles/ssd1306/pull/101) Add support for modules with a 72x40px display size. These are often advertised as 70x40px displays which are likely the same hardware. An example is also added - `graphics_i2c_72x40`.

### Fixed

- Fix docs.rs build by targeting `x86_64-unknown-linux-gnu`

### Changed

- **(breaking)** Upgrade embedded-graphics from `0.6.0-alpha.2` to version `0.6.0-alpha.3`
- [#106](https://github.com/jamwaffles/ssd1306/pull/106) Switch out some `for` loops for iterators internally to speed up data transfers and reduce code size in `--release` mode.

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

[unreleased]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.4...HEAD
[0.3.0-alpha.4]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.3...v0.3.0-alpha.4
[0.3.0-alpha.3]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.2...v0.3.0-alpha.3
[0.3.0-alpha.2]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.1...v0.3.0-alpha.2
[0.3.0-alpha.1]: https://github.com/jamwaffles/ssd1306/compare/0.2.5...v0.3.0-alpha.1
