//! SSD1306 Communication Interface (I2C/SPI)
//!
//! These are the two supported interfaces for communicating with the display. They're used by the
//! [builder](../builder/index.html) methods
//! [connect_i2c](../builder/struct.Builder.html#method.connect_i2c) and
//! [connect_spi](../builder/struct.Builder.html#method.connect_spi).
//!
//! The types that these interfaces define are quite lengthy, so it is recommended that you create
//! a type alias. Here's an example for the I2C1 on an STM32F103xx:
//!
//! ```rust
//! # use stm32f1xx_hal::gpio::gpiob::{PB8, PB9};
//! # use stm32f1xx_hal::gpio::{Alternate, OpenDrain};
//! # use stm32f1xx_hal::i2c::I2c;
//! # use stm32f1xx_hal::pac::I2C1;
//! # use stm32f1xx_hal::prelude::*;
//! use ssd1306::{interface::I2cInterface, prelude::*};
//! type OledDisplay =
//!   GraphicsMode<I2cInterface<I2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>>;
//! ```
//!
//! [Example](https://github.com/jamwaffles/ssd1306/blob/master/examples/blinky_i2c.rs)
//!
//! Here's one for SPI1 on an STM32F103xx:
//!
//! ```rust
//! # use stm32f1xx_hal::gpio::gpioa::{PA5, PA6, PA7};
//! # use stm32f1xx_hal::gpio::gpiob::PB1;
//! # use stm32f1xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
//! # use stm32f1xx_hal::pac::SPI1;
//! # use stm32f1xx_hal::prelude::*;
//! # use stm32f1xx_hal::spi::{Spi, Spi1NoRemap};
//! use ssd1306::{interface::SpiInterface, prelude::*};
//! pub type OledDisplay = GraphicsMode<
//!     SpiInterface<
//!         Spi<
//!             SPI1,
//!             Spi1NoRemap,
//!             (
//!                 PA5<Alternate<PushPull>>,
//!                 PA6<Input<Floating>>,
//!                 PA7<Alternate<PushPull>>,
//!             ),
//!         >,
//!         PB1<Output<PushPull>>,
//!     >,
//! >;
//! ```
//!
//! [Example](https://github.com/jamwaffles/ssd1306/blob/master/examples/blinky.rs)

pub mod i2c;
pub mod spi;

/// A method of communicating with SSD1306
pub trait DisplayInterface {
    /// Interface error type
    type Error;
    /// Send a batch of up to 8 commands to display.
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), Self::Error>;
    /// Send data to display.
    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    /// Send data to display, taking advantage of bounded data.
    ///
    /// upper_left and lower_right should contain the x and y coordinates of the
    /// minimum bounding rectangle of the modified pixels.
    fn send_bounded_data(
        &mut self,
        buf: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), Self::Error>;
}

pub use self::i2c::I2cInterface;
pub use self::spi::SpiInterface;
