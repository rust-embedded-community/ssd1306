//! Interface factory

use hal;
use hal::digital::OutputPin;

use super::displaysize::DisplaySize;
use super::displayrotation::DisplayRotation;
use super::interface::{I2cInterface, SpiInterface};
use super::properties::DisplayProperties;
use mode::raw::RawMode;

/// Communication interface factory
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
    i2c_addr: u8,
}

impl Builder {
    /// Create new builder for default size of 128 x 64 pixels.
    pub fn new() -> Self {
        Self {
            display_size: DisplaySize::Display128x64,
            rotation: DisplayRotation::Rotate0,
            i2c_addr: 0x3c,
        }
    }

    /// Create new builder for a specified size.
    pub fn with_size(&self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..*self
        }
    }

    /// Set the I2C address to use. Defaults to 0x3C which seems to be the most common address.
    /// The other address specified in the datasheet is 0x3D.
    ///
    /// Ignored when using SPI interface
    pub fn with_i2c_addr(&self, i2c_addr: u8) -> Self {
        Self { i2c_addr, ..*self }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation
    pub fn with_rotation(&self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..*self }
    }

    /// Create i2c communication interface
    pub fn connect_i2c<I2C>(&self, i2c: I2C) -> RawMode<I2cInterface<I2C>>
    where
        I2C: hal::blocking::i2c::Write,
    {
        let properties = DisplayProperties::new(
            I2cInterface::new(i2c, self.i2c_addr),
            self.display_size,
            self.rotation,
        );
        RawMode::new(properties)
    }

    /// Create spi communication interface
    pub fn connect_spi<SPI, DC>(&self, spi: SPI, dc: DC) -> RawMode<SpiInterface<SPI, DC>>
    where
        SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
        DC: OutputPin,
    {
        let properties =
            DisplayProperties::new(SpiInterface::new(spi, dc), self.display_size, self.rotation);
        RawMode::new(properties)
    }
}
