//! SSD1306 I2C Interface

use hal;

use super::DisplayInterface;
use crate::Error;
use display_interface::WriteOnlyDataCommand;
use display_interface_i2c::I2CInterface as DI2cInterface;

// TODO: Add to prelude
/// SSD1306 I2C communication interface
pub struct I2cInterface<I2C> {
    inner: DI2cInterface<I2C>,
}

impl<I2C, CommE> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
{
    /// Create new SSD1306 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            // 0x40 is the prefix for data mode
            // cf. 8.1.5.2 5) b) in the datasheet
            inner: DI2cInterface::new(i2c, addr, 0x40),
        }
    }
}

impl<I2C, CommE> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
{
    type Error = Error<CommE, ()>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.inner.send_commands(cmds).map_err(Error::Comm)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.inner.send_data(buf).map_err(Error::Comm)
    }
}
