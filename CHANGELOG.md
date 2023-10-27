# Changelog

[`ssd1306`](https://crates.io/crates/ssd1306) is a no-std driver written in Rust for the popular
SSD1306 monochrome OLED display.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.8.4] - 2023-10-27

### Fixed

- [#201](https://github.com/jamwaffles/ssd1306/pull/201) Fixed `BufferedGraphicsMode::clear(On)` such that it fills all pixels with `On`, not only some.

## [0.8.3] - 2023-10-09

### Changed

- [#195](https://github.com/jamwaffles/ssd1306/pull/195) Changed `BasicMode::clear` to clear in
  small batches instead of one big write. This drops RAM requirement by ~900b and fixes issues on
  MCUs with less than 1Kb of RAM.
- [#195](https://github.com/jamwaffles/ssd1306/pull/195) Changed `TerminalMode` to use lookup by
  ASCII code instead of per-character match when searching for glyph. This may save up to 3.5Kb of
  compiled code on AVR MCUs.

## [0.8.2] - 2023-09-29

### Fixed

- [#197](https://github.com/jamwaffles/ssd1306/pull/197) Fixed terminal mode panic and wrapping
  behaviour for rotated displays.

## [0.8.1] - 2023-08-18

### Added

- [#190](https://github.com/jamwaffles/ssd1306/pull/190) Added `Ssd1306::set_invert` to invert the
  screen pixels

## [0.8.0] - 2023-06-01

### Added

- [#183](https://github.com/jamwaffles/ssd1306/pull/183) `Brightness::custom()` is now publicly
  available.

### Fixed

- [#177](https://github.com/jamwaffles/ssd1306/pull/177) Fixed a few spelling mistakes.

### Changed

- **(breaking)** [#184](https://github.com/jamwaffles/ssd1306/pull/184) Increased MSRV to 1.61.0
- **(breaking)** [#179](https://github.com/jamwaffles/ssd1306/pull/179) Changed `Ssd1306::reset`
  signature.
- [#181](https://github.com/jamwaffles/ssd1306/pull/181) Update embedded-graphics-core dependency to
  0.4
- **(breaking)** [#185](https://github.com/jamwaffles/ssd1306/pull/185) The inherent
  `BufferedGraphicsMode::clear` has been renamed to `clear_buffer`.
- [#185](https://github.com/jamwaffles/ssd1306/pull/185) Some methods no longer require
  `DI: WriteOnlyDataCommand`.

## [0.7.1] - 2022-08-15

### Added

- [#161](https://github.com/jamwaffles/ssd1306/pull/161) Added a `set_mirror` method to enable or
  disable display mirroring.
- [#166](https://github.com/jamwaffles/ssd1306/pull/166) Added `DisplaySize` configuration for 64x32
  displays.

## [0.7.0] - 2021-07-08

### Changed

- **(breaking)** [#158](https://github.com/jamwaffles/ssd1306/pull/158) Migrate away from
  `generic-array` to a solution using const generics. This raises the crate MSRV to 1.51.

## [0.6.0] - 2021-06-22

### Changed

- **(breaking)** [#156](https://github.com/jamwaffles/ssd1306/pull/156) Migrate from
  `embedded-graphics` to `embedded-graphics-core`.
- **(breaking)** [#150](https://github.com/jamwaffles/ssd1306/pull/150)
  `BufferedGraphicsMode::set_pixel` now accepts a `bool` instead of a `u8` for the pixel color
  value.
- **(breaking)** [#150](https://github.com/jamwaffles/ssd1306/pull/150) `display_on` is now called
  `set_display_on`.
- **(breaking)** [#150](https://github.com/jamwaffles/ssd1306/pull/150) `TerminalMode::get_position`
  is now called `position` to conform with Rust API guidelines.
- **(breaking)** [#150](https://github.com/jamwaffles/ssd1306/pull/150) Refactor the crate API to be
  more versatile and to make code clearer to understand.

  A graphics mode initialisation now looks like this:

  ```rust
  use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

  let interface = I2CDisplayInterface::new(i2c);

  let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
      .into_buffered_graphics_mode();

  display.init().unwrap();
  ```

## [0.5.2] - 2021-04-19

- [#145](https://github.com/jamwaffles/ssd1306/pull/145) Fixed rotation for 96x16 and 72x40
  displays.
- [#147](https://github.com/jamwaffles/ssd1306/pull/147) Fixed display rotation in terminal mode.

## [0.5.1] - 2021-01-09

## [0.5.0] - 2020-12-21

## [0.4.2] - 2020-12-15 (yanked)

### Changed

- **(breaking)** [#139](https://github.com/jamwaffles/ssd1306/pull/139) Removed default display size
  type parameters.

### Fixed

- [#141](https://github.com/jamwaffles/ssd1306/pull/141) 72x40 displays can now be set to higher
  brightnesses, matching other sizes.

## [0.4.1] - 2020-12-01

### Changed

- [#137](https://github.com/jamwaffles/ssd1306/pull/137) Replaced `TerminalMode` font with a new,
  more consistent one. This now uses the `zxpix` font from
  <https://jared.geek.nz/2014/jan/custom-fonts-for-microcontrollers>.

## [0.4.0] - 2020-08-03

### Added

- [#121](https://github.com/jamwaffles/ssd1306/pull/121) Added brightness control with the
  `set_brightness()` method.
- [#118](https://github.com/jamwaffles/ssd1306/pull/118) `DisplayModeTrait::into_properties()` new
  method that consumes the driver and returns the `DisplayProperties`

### Changed

- **(breaking)** [#129](https://github.com/jamwaffles/ssd1306/pull/129) `TerminalMode::set_rotation`
  now resets the cursor position
- **(breaking)** [#125](https://github.com/jamwaffles/ssd1306/pull/125) Redesigned display size
  handling.
- **(breaking)** [#126](https://github.com/jamwaffles/ssd1306/pull/126) Moved `reset` method to
  `DisplayModeTrait`. If the prelude is not used, add either `use ssd1306::prelude::*` or
  `ssd1306::mode::displaymode::DisplayModeTrait` to your imports.
- **(breaking)** [#119](https://github.com/jamwaffles/ssd1306/pull/119) Remove `DisplayMode` and
  `RawMode`
- [#120](https://github.com/jamwaffles/ssd1306/pull/120) Update to v0.4
  [`display-interface`](https://crates.io/crates/display-interface)
- **(breaking)** [#118](https://github.com/jamwaffles/ssd1306/pull/118) Change `release` method to
  return the display interface instead of the `DisplayProperties`.
- **(breaking)** [#116](https://github.com/jamwaffles/ssd1306/pull/116) Replace custom I2C and SPI
  interfaces by generic [`display-interface`](https://crates.io/crates/display-interface)
- **(breaking)** [#113](https://github.com/jamwaffles/ssd1306/pull/113) Removed public
  `send_bounded_data` from DisplayInterface and implementations

### Fixed

- [#129](https://github.com/jamwaffles/ssd1306/pull/129) Fixed `Rotate90` and `Rotate270` rotation
  modes for `TerminalMode`

## [0.3.1] - 2020-03-21

### Fixed

- Fix docs.rs build config

## [0.3.0] - 2020-03-20

### Fixed

- [#111](https://github.com/jamwaffles/ssd1306/pull/111) Fix TerminalMode offset for smaller
  displays.

### Added

- [#111](https://github.com/jamwaffles/ssd1306/pull/111) Add support for modules with a 64x48px
  display size.

### Changed

- **(breaking)** [#112](https://github.com/jamwaffles/ssd1306/pull/112) Upgrade to embedded-graphics
  0.6.0
- [#107](https://github.com/jamwaffles/ssd1306/pull/107) Migrate from Travis to CircleCI
- [#105](https://github.com/jamwaffles/ssd1306/pull/105) Reduce flash usage by around 400 bytes by
  replacing some internal `unwrap()`s with `as` coercions.
- [#106](https://github.com/jamwaffles/ssd1306/pull/106) Optimise internals by using iterators to
  elide bounds checks. Should also speed up `GraphicsMode` (and `embedded-graphics` operations) with
  a cleaned-up `set_pixel`.
- [#108](https://github.com/jamwaffles/ssd1306/pull/108) Add an example using
  `DisplayProperties.draw()` to send a raw buffer of random bytes to the display over I2C.

### Added

- [#110](https://github.com/jamwaffles/ssd1306/pull/110) Add an animated image example `rtfm_dvd`
  using [RTFM](https://crates.io/crates/cortex-m-rtfm)

## [0.3.0-alpha.4] - 2020-02-07

### Added

- [#101](https://github.com/jamwaffles/ssd1306/pull/101) Add support for modules with a 72x40px
  display size. These are often advertised as 70x40px displays which are likely the same hardware.
  An example is also added - `graphics_i2c_72x40`.

### Fixed

- Fix docs.rs build by targeting `x86_64-unknown-linux-gnu`

### Changed

- **(breaking)** Upgrade embedded-graphics from `0.6.0-alpha.2` to version `0.6.0-alpha.3`
- [#106](https://github.com/jamwaffles/ssd1306/pull/106) Switch out some `for` loops for iterators
  internally to speed up data transfers and reduce code size in `--release` mode.

## [0.3.0-alpha.3] - 2020-02-03

### Changed

- [#97](https://github.com/jamwaffles/ssd1306/pull/97) Use the new `Triangle` primitive from
  Embedded Graphics 0.6.0-alpha.2 in the three SSD1306 `graphics*.rs` examples
- [#103](https://github.com/jamwaffles/ssd1306/pull/103) Pin embedded-graphics version to
  0.6.0-alpha.2

## [0.3.0-alpha.2]

### Fixed

- [#80](https://github.com/jamwaffles/ssd1306/pull/80) `TerminalMode` now has a cursor. Newlines
  (`\n`) and carriage returns (`\r`) are now supported.

### Changed

- [#94](https://github.com/jamwaffles/ssd1306/pull/94) Programs that only change some parts of the
  display should now run much faster. The driver keeps track of changed pixels and only sends a
  bounding box of updated pixels instead of updating the entire display every time.

## [0.3.0-alpha.1]

### Changed

- **(breaking)** Upgraded to Embedded Graphics 0.6.0-alpha.2

## 0.2.6

### Added

- Added a changelog!
- Display power control (#86) - call `.display_on(true)` or `.display_on(false)` to turn on or off
  the display respectively.

### Fixed

- Doc examples are now tested in CI (#89)

### Changed

- Builder docs clarify the order of method calls (#89)

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/ssd1306/compare/v0.8.4...HEAD
[0.8.4]: https://github.com/jamwaffles/ssd1306/compare/v0.8.3...v0.8.4

[0.8.3]: https://github.com/jamwaffles/ssd1306/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/jamwaffles/ssd1306/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/jamwaffles/ssd1306/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/jamwaffles/ssd1306/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/jamwaffles/ssd1306/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/jamwaffles/ssd1306/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/jamwaffles/ssd1306/compare/v0.5.2...v0.6.0
[0.5.2]: https://github.com/jamwaffles/ssd1306/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/jamwaffles/ssd1306/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/jamwaffles/ssd1306/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/jamwaffles/ssd1306/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/jamwaffles/ssd1306/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/jamwaffles/ssd1306/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.4...v0.3.0
[0.3.0-alpha.4]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.3...v0.3.0-alpha.4
[0.3.0-alpha.3]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.2...v0.3.0-alpha.3
[0.3.0-alpha.2]: https://github.com/jamwaffles/ssd1306/compare/v0.3.0-alpha.1...v0.3.0-alpha.2
[0.3.0-alpha.1]: https://github.com/jamwaffles/ssd1306/compare/0.2.5...v0.3.0-alpha.1
