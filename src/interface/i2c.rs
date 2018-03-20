//! SSD1306 I2C Interface

use hal;

use super::DisplayInterface;

/// SSD1306 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    /// Create new SSD1306 I2C interface
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    type Error = I2C::Error;

    fn send_command(&mut self, cmd: u8) -> Result<(), I2C::Error> {
        self.i2c.write(0x3c, &[0, cmd])?;

        Ok(())
    }

    // TODO: Send data in chunks to save memory. This code is particularly bad with 128x32 displays
    // as half of `writebuf` is completely wasted.
    fn send_data(&mut self, buf: &[u8]) -> Result<(), I2C::Error> {
        let mut writebuf: [u8; 1025] = [0; 1025];

        // Data mode
        // 8.1.5.2 5) b) in the datasheet
        writebuf[0] = 0x40;

        for (index, byte) in buf.iter().enumerate() {
            writebuf[index + 1] = *byte;
        }

        self.i2c.write(0x3c, &writebuf[0..buf.len() + 1])?;

        Ok(())
    }
}
