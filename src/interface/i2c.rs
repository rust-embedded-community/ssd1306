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

        buf.chunks(16).try_for_each(|c| {
            let chunk_len = c.len();

            // Copy over all data from buffer, leaving the data command byte intact
            writebuf[1..=chunk_len].copy_from_slice(c);

            self.i2c
                .write(self.addr, &writebuf[0..=chunk_len])
                .map_err(Error::Comm)
        })
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

        // Write buffer. Writes are sent in chunks of 16 bytes plus DC byte
        let mut writebuf: [u8; 17] = [0x0; 17];

        // Data mode
        // 8.1.5.2 5) b) in the datasheet
        writebuf[0] = 0x40;

        // Divide by 8 since each row is actually 8 pixels tall
        let num_pages = ((lower_right.1 - upper_left.1) / 8) as usize + 1;

        // Each page is 8 bits tall, so calculate which page number to start at (rounded down) from
        // the top of the display
        let starting_page = (upper_left.1 / 8) as usize;

        // Calculate start and end X coordinates for each page
        let page_lower = upper_left.0 as usize;
        let page_upper = lower_right.0 as usize;

        buf.chunks(disp_width)
            .skip(starting_page)
            .take(num_pages)
            .map(|s| &s[page_lower..page_upper])
            .try_for_each(|c| {
                c.chunks(16).try_for_each(|c| {
                    let chunk_len = c.len();

                    // Copy over all data from buffer, leaving the data command byte intact
                    writebuf[1..=chunk_len].copy_from_slice(c);

                    self.i2c
                        .write(self.addr, &writebuf[0..=chunk_len])
                        .map_err(Error::Comm)
                })
            })
    }
}
