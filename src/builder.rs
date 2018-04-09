//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation and size (128x64):
//!
//! ```rust,ignore
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! Builder::new().connect_spi(spi, dc);
//! ```
//!
//! Connect over I2C with lots of options
//!
//! ```rust,ignore
//! let i2c = /* I2C interface from your HAL of choice */;
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .with_i2c_addr(0x3D)
//!     .with_size(DisplaySize::Display128x32)
//!     .connect_i2c(i2c);
//! ```

use hal;
use hal::digital::OutputPin;

use super::displayrotation::DisplayRotation;
use super::displaysize::DisplaySize;
use super::interface::{I2cInterface, SpiInterface};
use super::properties::DisplayProperties;
use mode::displaymode::DisplayMode;
use mode::raw::RawMode;

/// Driver builder
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
    i2c_addr: u8,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new() -> Self {
        Self {
            display_size: DisplaySize::Display128x64,
            rotation: DisplayRotation::Rotate0,
            i2c_addr: 0x3c,
        }
    }

    /// Set the size of the display. Supported sizes are defined by [DisplaySize]
    pub fn with_size(&self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..*self
        }
    }

    /// Set the I2C address to use. Defaults to 0x3C which seems to be the most common address.
    /// The other address specified in the datasheet is 0x3D. Ignored when using SPI interface.
    pub fn with_i2c_addr(&self, i2c_addr: u8) -> Self {
        Self { i2c_addr, ..*self }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation
    pub fn with_rotation(&self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..*self }
    }

    /// Finish the builder and use I2C to communicate with the display
    pub fn connect_i2c<I2C>(&self, i2c: I2C) -> DisplayMode<RawMode<I2cInterface<I2C>>>
    where
        I2C: hal::blocking::i2c::Write,
    {
        let properties = DisplayProperties::new(
            I2cInterface::new(i2c, self.i2c_addr),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<I2cInterface<I2C>>>::new(properties)
    }

    /// Finish the builder and use SPI to communicate with the display
    pub fn connect_spi<SPI, DC>(
        &self,
        spi: SPI,
        dc: DC,
    ) -> DisplayMode<RawMode<SpiInterface<SPI, DC>>>
    where
        SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
        DC: OutputPin,
    {
        let properties =
            DisplayProperties::new(SpiInterface::new(spi, dc), self.display_size, self.rotation);
        DisplayMode::<RawMode<SpiInterface<SPI, DC>>>::new(properties)
    }
}
