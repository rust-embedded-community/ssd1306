//! SSD1306 OLED display driver
//!
//! The driver must be initialised by passing an I2C or SPI interface peripheral to the [`Builder`],
//! which will in turn create a driver instance in a particular mode. By default, the builder
//! returns a [`DisplayProperties`] instance which is a low level interface to manipulate the
//! display properties (e.g. rotation). The driver can be coerced into a more useful mode by calling
//! `into()` and defining the type you want to coerce to. For example, to initialise the display
//! with an I2C interface and [`GraphicsMode`], you would do something like this:
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub as I2cInterface;
//! use ssd1306::{mode::GraphicsMode, Builder, I2CDIBuilder};
//!
//! // Configure an I2C interface on the target device; below line shown as example only
//! let i2c = I2cInterface;
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let mut disp: GraphicsMode<_, _> = Builder::new().connect(interface).into();
//! disp.init();
//!
//! disp.set_pixel(10, 20, 1);
//! ```
//!
//! See the [example](https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics_i2c.rs)
//! for more usage. The entire `embedded_graphics` [featureset]
//! is supported by this driver.
//!
//! There is also [`TerminalMode`] which allows drawing of characters to the display without
//! using a display buffer:
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub as I2cInterface;
//! use ssd1306::{mode::TerminalMode, Builder, I2CDIBuilder};
//!
//! // Configure an I2C interface on the target device; below line shown as example only
//! let i2c = I2cInterface;
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
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
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use core::fmt::Write;
//! use ssd1306::{mode::TerminalMode, prelude::*, Builder, I2CDIBuilder};
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
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
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! use ssd1306::{mode::GraphicsMode, prelude::*, Builder, I2CDIBuilder};
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let mut disp: GraphicsMode<_, _> = Builder::new().connect(interface).into();
//!
//! disp.init().unwrap();
//!
//! let text_style = TextStyleBuilder::new(Font6x8)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! Text::new("Hello world!", Point::zero())
//!     .into_styled(text_style)
//!     .draw(&mut disp);
//!
//! Text::new("Hello Rust!", Point::new(0, 16))
//!     .into_styled(text_style)
//!     .draw(&mut disp);
//!
//! disp.flush().unwrap();
//! ```
//!
//! [featureset]: https://github.com/jamwaffles/embedded-graphics#features
//! [`Builder`]: ./builder/struct.Builder.html
//! [`DisplayProperties`]: ./properties/struct.DisplayProperties.html
//! [`GraphicsMode`]: ./mode/graphics/struct.GraphicsMode.html
//! [`TerminalMode`]: ./mode/terminal/struct.TerminalMode.html

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

pub mod brightness;
pub mod builder;
pub mod command;
pub mod displayrotation;
pub mod displaysize;
pub mod mode;
pub mod prelude;
pub mod properties;
#[doc(hidden)]
pub mod test_helpers;

pub use crate::builder::Builder;
pub use crate::builder::I2CDIBuilder;
