use hal;

use super::DisplayInterface;

pub struct I2cInterface<I2C> {
    i2c: I2C,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    fn send_command(&mut self, cmd: u8) {
        self.i2c.write(0x3c, &[0, cmd]);
    }

    // FIXME: I should be able to move all this command rubbish up into the SSD1306 impl. Hmm.
    fn send_data(&mut self, buf: &[u8]) {
        let flush_commands: [u8; 6] = [
            0x21, // Set column address from addr...
            0,    // 0 to ...
            127,  // 128 columns (0 indexed).

            0x22, // Set pages from addr ...
            0,    // 0 to ...
            7     // 8 pages (0 indexed). 8 pages of 8 rows (1 byte) each = 64px high
        ];

        for c in flush_commands.iter() {
            self.send_command(*c);
        }

        let mut writebuf: [u8; 1025] = [0; 1025];

        // Data mode
        // 8.1.5.2 5) b) in the datasheet
        writebuf[0] = 0x40;

        for (index, byte) in buf.iter().enumerate() {
            writebuf[index + 1] = *byte;
        }

        self.i2c.write(0x3c, &writebuf);
    }
}
