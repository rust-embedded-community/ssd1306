use hal;
use hal::digital::OutputPin;

use super::DisplayInterface;

pub struct SpiInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    DC: OutputPin,
{
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    DC: OutputPin,
{
    fn send_command(&mut self, cmd: u8) {
        self.dc.set_low();

        self.spi.write(&[cmd]);

        self.dc.set_high();
    }

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

        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf);
    }
}
