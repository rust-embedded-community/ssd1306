//! SSD1306 OLED display driver
//!
//! The driver must be initialised by passing an I2C or SPI interface peripheral to the [`Builder`],
//! which will in turn create a driver instance in a particular mode. By default, the builder
//! returns a [`RawMode`] instance which isn't very useful by itself. You can coerce the driver
//! into a more useful mode by calling `into()` and defining the type you want to coerce to. For
//! example, to initialise the display with an I2C interface and [`GraphicsMode`], you would do
//! something like this:
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub as I2cInterface;
//! use ssd1306::{Builder, mode::GraphicsMode};
//!
//! // Configure an I2C interface on the target device; below line shown as example only
//! let i2c = I2cInterface;
//!
//! let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//! disp.init();
//!
//! disp.set_pixel(10, 20, 1);
//! ```
//!
//! See the [example](https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics_i2c.rs)
//! for more usage. The [entire `embedded_graphics` featureset](https://github.com/jamwaffles/embedded-graphics#features)
//! is supported by this driver.
//!
//! There is also [`TerminalMode`] which allows drawing of characters to the display without
//! using a display buffer:
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub as I2cInterface;
//! use ssd1306::{mode::TerminalMode, Builder};
//!
//! // Configure an I2C interface on the target device; below line shown as example only
//! let i2c = I2cInterface;
//!
//! let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! disp.print_char('A');
//! ```
//!
//! See the [example](https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs)
//! for more usage.
//!
//! It's possible to customise the driver to suit your display/application. Take a look at the
//! [`Builder`] for available options.
//!
//! # Examples
//!
//! Examples can be found in
//! [the examples/ folder](https://github.com/jamwaffles/ssd1306/blob/master/examples)
//!
//! ## Write text to the display without a framebuffer
//!
//! Uses [`TerminalMode`]. [See the complete example
//! here](https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs).
//!
//! ```rust
//!	# use ssd1306::test_helpers::I2cStub;
//!	# let i2c = I2cStub;
//! use core::fmt::Write;
//! use ssd1306::{prelude::*, mode::TerminalMode, Builder};
//!
//! let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
//! disp.init().unwrap();
//! let _ = disp.clear();
//!
//! // Spam some characters to the display
//! for c in 97..123 {
//!     let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//! }
//! for c in 65..91 {
//!     let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//! }
//! ```
//!
//! ## Draw some text to the display
//!
//! Uses [`GraphicsMode`] and [embedded_graphics](../embedded_graphics/index.html). [See the
//! complete example here](https://github.com/jamwaffles/ssd1306/blob/master/examples/text_i2c.rs).
//!
//! ```rust
//!	# use ssd1306::test_helpers::I2cStub;
//!	# let i2c = I2cStub;
//! use ssd1306::{prelude::*, mode::GraphicsMode, Builder};
//! use embedded_graphics::{prelude::*, fonts::Font6x8};
//!
//! let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! disp.init().unwrap();
//! disp.flush().unwrap();
//!
//! disp.draw(
//!     Font6x8::render_str("Hello world!")
//!         .with_stroke(Some(1u8.into()))
//!         .into_iter(),
//! );
//! disp.draw(
//!     Font6x8::render_str("Hello Rust!")
//!         .with_stroke(Some(1u8.into()))
//!         .translate(Coord::new(0, 16))
//!         .into_iter(),
//! );
//!
//! disp.flush().unwrap();
//! ```
//!
//! [`Builder`]: ./builder/struct.Builder.html
//! [`GraphicsMode`]: ./mode/graphics/struct.GraphicsMode.html
//! [`TerminalMode`]: ./mode/terminal/struct.TerminalMode.html
//! [`RawMode`]: ./mode/raw/struct.RawMode.html

#![no_std]
// #![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
mod displaysize;
pub mod interface;
pub mod mode;
pub mod prelude;
pub mod properties;
#[doc(hidden)]
pub mod test_helpers;

pub use crate::builder::Builder;
