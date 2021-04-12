//! I2C interface factory

/// Builder struct for an I2C interface. Driver options and interface are set using its methods.
#[derive(Copy, Clone, Debug)]
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
    pub fn init<I: embedded_hal::blocking::i2c::Write>(
        self,
        i2c: I,
    ) -> display_interface_i2c::I2CInterface<I> {
        display_interface_i2c::I2CInterface::new(i2c, self.i2c_addr, 0x40)
    }
}
