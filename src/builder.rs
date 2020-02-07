//! Interface factory
//!
//! This is the easiest way to create a driver instance, with the ability to set various parameters of the driver.
//!
//! To finish the builder and produce a connected display interface, call `.connect_i2c(i2c)` or
//! `.connect_spi(spi, dc)`. The builder will be consumed into a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which can be coerced into a richer
//! display mode like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, SpiStub};
//! # let spi = SpiStub;
//! # let dc = PinStub;
//! use ssd1306::Builder;
//!
//! Builder::new().connect_spi(spi, dc);
//! ```
//!
//! Connect over I2C, changing lots of options
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, I2cStub};
//! # let i2c = I2cStub;
//! use ssd1306::{prelude::*, Builder};
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .with_i2c_addr(0x3D)
//!     .size(DisplaySize::Display128x32)
//!     .connect_i2c(i2c);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`TerminalMode` mode](../mode/terminal/struct.TerminalMode.html):
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, SpiStub};
//! # let spi = SpiStub;
//! # let dc = PinStub;
//! use ssd1306::{prelude::*, Builder};
//!
//! let display: TerminalMode<_> = Builder::new().connect_spi(spi, dc).into();
//! ```

use hal::{self, digital::v2::OutputPin};

use crate::{
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    interface::{I2cInterface, SpiInterface},
    mode::{displaymode::DisplayMode, raw::RawMode},
    properties::DisplayProperties,
};

/// Builder struct. Driver options and interface are set using its methods.
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

    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn size(&self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..*self
        }
    }

    /// Set the I2C address to use. Defaults to 0x3C which is the most common address.
    /// The other address specified in the datasheet is 0x3D. Ignored when using SPI interface.
    pub fn with_i2c_addr(&self, i2c_addr: u8) -> Self {
        Self { i2c_addr, ..*self }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation. Note that
    /// 90º and 270º rotations are not supported by
    /// [`TerminalMode`](../mode/terminal/struct.TerminalMode.html).
    pub fn with_rotation(&self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..*self }
    }

    /// Finish the builder and use I2C to communicate with the display
    ///
    /// This method consumes the builder and must come last in the method call chain
    pub fn connect_i2c<I2C, CommE>(&self, i2c: I2C) -> DisplayMode<RawMode<I2cInterface<I2C>>>
    where
        I2C: hal::blocking::i2c::Write<Error = CommE>,
    {
        let properties = DisplayProperties::new(
            I2cInterface::new(i2c, self.i2c_addr),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<I2cInterface<I2C>>>::new(properties)
    }

    /// Finish the builder and use SPI to communicate with the display
    ///
    /// This method consumes the builder and must come last in the method call chain
    pub fn connect_spi<SPI, DC, CommE, PinE>(
        &self,
        spi: SPI,
        dc: DC,
    ) -> DisplayMode<RawMode<SpiInterface<SPI, DC>>>
    where
        SPI: hal::blocking::spi::Transfer<u8, Error = CommE>
            + hal::blocking::spi::Write<u8, Error = CommE>,
        DC: OutputPin<Error = PinE>,
    {
        let properties =
            DisplayProperties::new(SpiInterface::new(spi, dc), self.display_size, self.rotation);
        DisplayMode::<RawMode<SpiInterface<SPI, DC>>>::new(properties)
    }
}
