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
        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf);
    }
}
