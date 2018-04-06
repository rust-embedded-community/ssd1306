//! SSD1306 I2C Interface

use hal;

use super::DisplayInterface;

/// SSD1306 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    /// Create new SSD1306 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    type Error = I2C::Error;

    fn send_command(&mut self, cmd: u8) -> Result<(), I2C::Error> {
        self.i2c.write(self.addr, &[0, cmd])?;

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), I2C::Error> {
        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        for chunk in buf.chunks(16) {
            let writebuf = {
                let mut writebuf: [u8; 17] = Default::default();

                // Data mode
                // 8.1.5.2 5) b) in the datasheet
                writebuf[0] = 0x40;

                writebuf
                    .iter_mut()
                    .skip(1)
                    .zip(chunk)
                    .for_each(|(dst, src)| *dst = *src);

                writebuf
            };

            self.i2c.write(self.addr, &writebuf[..1 + chunk.len()])?;
        }

        Ok(())
    }
}
