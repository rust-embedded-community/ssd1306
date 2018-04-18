//! SSD1306 OLED display driver
//!
//! The driver must be initialised by passing an I2C or SPI interface peripheral to the [`Builder`],
//! which will in turn create a driver instance in a particular mode. By default, the builder
//! returns a [`mode::RawMode`] instance which isn't very useful by itself. You can coerce the driver
//! into a more useful mode by calling `into()` and defining the type you want to coerce to. For
//! example, to initialise the display with an I2C interface and [mode::GraphicsMode], you would do
//! something like this:
//!
//! ```rust,ignore
//! let i2c = I2c::i2c1(/* snip */);
//!
//! let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//! disp.init();
//!
//! disp.set_pixel(10, 20, 1);
//! ```
//!
//! There is also [mode::TerminalMode] which allows drawing of characters to the display without
//! using a display buffer:
//!
//! ```rust,ignore
//! let i2c = I2c::i2c1(/* snip */);
//!
//! let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! disp.print_char('A');
//! ```
//!
//! It's possible to customise the driver to suit your display/application. Take a look at the
//! [Builder] for available options.

#![no_std]
// TODO: Docs
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

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
mod displaysize;
pub mod interface;
pub mod mode;
pub mod properties;

pub use builder::Builder;
pub use displaysize::DisplaySize;
