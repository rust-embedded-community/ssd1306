//! I2C interface factory

use display_interface_i2c::I2CInterface;

/// Helper struct to create preconfigured I2C interfaces for the display.
#[derive(Debug, Copy, Clone)]
pub struct I2CDisplayInterface(());

impl I2CDisplayInterface {
    /// Create new builder with a default I2C address of 0x3C
    #[allow(clippy::new_ret_no_self)]
    // pub fn with_i2c<I>(i2c: I) -> I2CInterface<I> // alternative, but breaking change
    pub fn new<I>(i2c: I) -> I2CInterface<I>
    where
        I: embedded_hal::i2c::I2c,
    {
        Self::new_custom_address(i2c, 0x3C)
    }

    /// Create a new I2C interface with the alternate address 0x3D as specified in the datasheet.
    pub fn new_alternate_address<I>(i2c: I) -> I2CInterface<I>
    where
        I: embedded_hal::i2c::I2c,
    {
        Self::new_custom_address(i2c, 0x3D)
    }

    /// Create a new I2C interface with a custom address.
    pub fn new_custom_address<I>(i2c: I, address: u8) -> I2CInterface<I>
    where
        I: embedded_hal::i2c::I2c,
    {
        I2CInterface::new(i2c, address, 0x40)
    }
}
