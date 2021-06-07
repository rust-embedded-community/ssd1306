//! I2C interface factory

use display_interface_i2c::I2CInterface;

/// Helper struct to create preconfigured I2C interfaces for the display.
#[derive(Debug, Copy, Clone)]
pub struct I2CDisplayInterface(());

impl I2CDisplayInterface {
    /// Create new builder with a default I2C address of 0x3C
    pub fn new<I>(i2c: I) -> I2CInterface<I>
    where
        I: embedded_hal::blocking::i2c::Write,
    {
        Self::new_custom_address(i2c, 0x3C)
    }

    /// Create a new I2C interface with the alternate address 0x3D as specified in the datasheet.
    pub fn new_alternate_address<I>(i2c: I) -> I2CInterface<I>
    where
        I: embedded_hal::blocking::i2c::Write,
    {
        Self::new_custom_address(i2c, 0x3D)
    }

    /// Create a new I2C interface with a custom address.
    pub fn new_custom_address<I>(i2c: I, address: u8) -> I2CInterface<I>
    where
        I: embedded_hal::blocking::i2c::Write,
    {
        I2CInterface::new(i2c, address, 0x40)
    }
}
