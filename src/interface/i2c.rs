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

    fn send_bounded_data(
        &mut self,
        buf: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), Self::Error> {
        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        let mut writebuf: [u8; 17] = [0; 17];

        // Divide by 8 since each row is actually 8 pixels tall
        let height = ((lower_right.1 - upper_left.1) / 8) as usize;

        let starting_page = (upper_left.1 / 8) as usize;

        // Data mode
        // 8.1.5.2 5) b) in the datasheet
        writebuf[0] = 0x40;

        let mut page_offset = starting_page * disp_width;

        for _ in 0..=height {
            let start_index = page_offset + upper_left.0 as usize;
            let end_index = page_offset + lower_right.0 as usize;

            page_offset += disp_width;

            let sub_buf = &buf[start_index..end_index];

            for chunk in sub_buf.chunks(16) {
                let chunklen = chunk.len();

                // Copy over all data from buffer, leaving the data command byte intact
                writebuf[1..=chunklen].copy_from_slice(&chunk[0..chunklen]);

                self.i2c
                    .write(self.addr, &writebuf[..=chunklen])
                    .map_err(Error::Comm)?;
            }
        }

        Ok(())
    }
}
