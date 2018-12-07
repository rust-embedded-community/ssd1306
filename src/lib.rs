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
//! See the [example](https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics_i2c.rs)
//! for more usage. The [entire `embedded_graphics` featureset](https://github.com/jamwaffles/embedded-graphics#features)
//! is supported by this driver.
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
//! See the [example](https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs)
//! for more usage.
//!
//! It's possible to customise the driver to suit your display/application. Take a look at the
//! [Builder] for available options.
//!
//! # Examples
//!
//! Examples can be found in
//! [the examples/ folder](https://github.com/jamwaffles/ssd1306/blob/master/examples)
//!
//! ## Write text to the display without a framebuffer
//!
//! Uses [mode::TerminalMode].
//!
//! ```rust,no-run
//! #![no_std]
//!
//! extern crate cortex_m;
//! extern crate embedded_hal as hal;
//! extern crate panic_abort;
//! extern crate ssd1306;
//! extern crate stm32f103xx_hal as blue_pill;
//!
//! use blue_pill::i2c::{DutyCycle, I2c, Mode};
//! use blue_pill::prelude::*;
//! use core::fmt::Write;
//! use ssd1306::{mode::TerminalMode, Builder};
//!
//! fn main() {
//!     let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
//!     let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
//!     let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
//!     let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
//!     let i2c = I2c::i2c1(
//!         dp.I2C1,
//!         (scl, sda),
//!         &mut afio.mapr,
//!         Mode::Fast {
//!             frequency: 400_000,
//!             duty_cycle: DutyCycle::Ratio1to1,
//!         },
//!         clocks,
//!         &mut rcc.apb1,
//!     );
//!
//!     let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
//!     disp.init().unwrap();
//!     disp.clear().unwrap();
//!
//!     // Endless loop
//!     loop {
//!         for c in 97..123 {
//!             let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//!         }
//!         for c in 65..91 {
//!             let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//!         }
//!     }
//! }
//! ```
//!
//! ## Draw some text to the display
//!
//! Uses [mode::GraphicsMode] and [embedded_graphics](../embedded_graphics/index.html).
//!
//! ```rust,no-run
//! #![no_std]
//!
//! extern crate cortex_m;
//! extern crate embedded_graphics;
//! extern crate embedded_hal as hal;
//! extern crate panic_abort;
//! extern crate ssd1306;
//! extern crate stm32f103xx_hal as blue_pill;
//!
//! use blue_pill::i2c::{DutyCycle, I2c, Mode};
//! use blue_pill::prelude::*;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::prelude::*;
//! use ssd1306::{mode::GraphicsMode, Builder};
//!
//! fn main() {
//!     let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
//!     let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
//!     let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
//!     let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
//!
//!     let i2c = I2c::i2c1(
//!         dp.I2C1,
//!         (scl, sda),
//!         &mut afio.mapr,
//!         Mode::Fast {
//!             frequency: 400_000,
//!             duty_cycle: DutyCycle::Ratio1to1,
//!         },
//!         clocks,
//!         &mut rcc.apb1,
//!     );
//!
//!     let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//!     disp.init().unwrap();
//!     disp.flush().unwrap();
//!     disp.draw(Font6x8::render_str("Hello world!", 1u8.into()).into_iter());
//!     disp.draw(
//!         Font6x8::render_str("Hello Rust!")
//!             .translate(Coord::new(0, 16))
//!             .into_iter(),
//!     );
//!     disp.flush().unwrap();
//! }
//! ```

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
pub mod prelude;
pub mod properties;

pub use crate::builder::Builder;
