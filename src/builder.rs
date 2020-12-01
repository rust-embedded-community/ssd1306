//! Interface factory
//!
//! This is the easiest way to create a driver instance, with the ability to set various parameters
//! of the driver.
//!
//! To finish the builder and produce a connected display interface, call `.connect(interface)`
//! where `interface` is an instantiated `DisplayInterface` implementation. For I2C interfaces
//! there's also an [`I2CDIBuilder`] to simplify the construction of an I2C `DisplayInterface`. The
//! builder will be consumed into a [`DisplayProperties`] object which can be coerced into a richer
//! display mode like [`GraphicsMode`] or [`TerminalMode`].
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
//! let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
//! Builder::new().connect(interface);
//! ```
//!
//! Connect over I2C, changing lots of options
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, I2cStub};
//! # let i2c = I2cStub;
//! use ssd1306::{prelude::*, Builder, I2CDIBuilder};
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let di: DisplayProperties<_, _> = Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .connect(interface);
//! ```
//!
//! The builder defaults to a display size of 128 x 64px. To use a display with a different size,
//! call the [`size`](#method.size) method. Supported sizes can be found in the
//! [`displaysize`](../displaysize/index.html) module or in the [prelude](../prelude/index.html).
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, I2cStub};
//! # let i2c = I2cStub;
//! use ssd1306::{prelude::*, Builder, I2CDIBuilder};
//!
//! let interface = I2CDIBuilder::new().init(i2c);
//! let di: DisplayProperties<_, _> = Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .size(DisplaySize128x32)
//!     .connect(interface);
//! ```
//!
//! The above examples will produce a [`DisplayProperties`] instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`TerminalMode`] mode:
//!
//! ```rust
//! # use ssd1306::test_helpers::{PinStub, SpiStub};
//! # let spi = SpiStub;
//! # let dc = PinStub;
//! use ssd1306::{prelude::*, Builder};
//!
//! let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
//! let display: TerminalMode<_, _> = Builder::new().connect(interface).into();
//! ```
//!
//! [`I2CDIBuilder`]: ./struct.I2CDIBuilder.html
//! [`DisplayProperties`]: ../properties/struct.DisplayProperties.html
//! [`GraphicsMode`]: ../mode/graphics/struct.GraphicsMode.html
//! [`TerminalMode`]: ../mode/terminal/struct.TerminalMode.html

use display_interface::WriteOnlyDataCommand;

use crate::{displayrotation::DisplayRotation, displaysize::*, properties::DisplayProperties};

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder<DSIZE = DisplaySize128x64>
where
    DSIZE: DisplaySize,
{
    size: DSIZE,
    rotation: DisplayRotation,
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
            size: DisplaySize128x64,
            rotation: DisplayRotation::Rotate0,
        }
    }
}

impl<DSIZE> Builder<DSIZE>
where
    DSIZE: DisplaySize,
{
    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn size<S: DisplaySize>(self, size: S) -> Builder<S> {
        Builder {
            size,
            rotation: self.rotation,
        }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation.
    pub fn with_rotation(self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..self }
    }

    /// Finish the builder and use some interface communicate with the display
    ///
    /// This method consumes the builder and must come last in the method call chain.
    ///
    /// ```rust
    /// # use ssd1306::test_helpers::{PinStub, I2cStub};
    /// # let i2c = I2cStub;
    /// use ssd1306::{prelude::*, Builder, I2CDIBuilder};
    ///
    /// let interface = I2CDIBuilder::new().init(i2c);
    /// let di: DisplayProperties<_, _> = Builder::new()
    ///     .size(DisplaySize128x32)
    ///     .connect(interface);
    /// ```
    pub fn connect<I>(self, interface: I) -> DisplayProperties<I, DSIZE>
    where
        I: WriteOnlyDataCommand,
    {
        DisplayProperties::new(interface, self.size, self.rotation)
    }
}

/// Builder struct for an I2C interface. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct I2CDIBuilder {
    i2c_addr: u8,
}

impl Default for I2CDIBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl I2CDIBuilder {
    /// Create new builder with a default I2C address of 0x3C
    pub fn new() -> Self {
        Self { i2c_addr: 0x3c }
    }

    /// Set the I2C address to use
    ///
    /// [`I2CDIBuilder`] defaults to an address of `0x3C` which is the most common address.
    /// The other address specified in the datasheet is `0x3D` which can be set using this method.
    pub fn with_i2c_addr(self, i2c_addr: u8) -> Self {
        Self { i2c_addr }
    }

    /// Finish the builder and return an initialised display interface for further use
    ///
    /// This method consumes the builder and must come last in the method call chain.
    pub fn init<I: hal::blocking::i2c::Write>(
        self,
        i2c: I,
    ) -> display_interface_i2c::I2CInterface<I> {
        display_interface_i2c::I2CInterface::new(i2c, self.i2c_addr, 0x40)
    }
}
