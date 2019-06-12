//! SSD1306 I2C Interface

use hal;

use super::DisplayInterface;
use crate::Error;

// TODO: Add to prelude
/// SSD1306 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, CommE> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
{
    /// Create new SSD1306 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C, CommE> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = CommE>,
{
    type Error = Error<CommE, ()>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        // Copy over given commands to new aray to prefix with command identifier
        let mut writebuf: [u8; 8] = [0; 8];
        writebuf[1..=cmds.len()].copy_from_slice(&cmds[0..cmds.len()]);

        self.i2c
            .write(self.addr, &writebuf[..=cmds.len()])
            .map_err(Error::Comm)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        let mut writebuf: [u8; 17] = [0; 17];

        // Data mode
        // 8.1.5.2 5) b) in the datasheet
        writebuf[0] = 0x40;

        for chunk in buf.chunks(16) {
            let chunklen = chunk.len();

            // Copy over all data from buffer, leaving the data command byte intact
            writebuf[1..=chunklen].copy_from_slice(&chunk[0..chunklen]);

            self.i2c
                .write(self.addr, &writebuf[..=chunklen])
                .map_err(Error::Comm)?;
        }

        Ok(())
    }
}
