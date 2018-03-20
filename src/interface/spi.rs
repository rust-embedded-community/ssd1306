//! SSD1306 SPI interface

use hal;
use hal::digital::OutputPin;

use super::DisplayInterface;

/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8>,
    DC: OutputPin,
{
    /// Create new SPI interface for communciation with SSD1306
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8>,
    DC: OutputPin,
{
    type Error = SPI::Error;

    fn send_command(&mut self, cmd: u8) -> Result<(), SPI::Error> {
        self.dc.set_low();

        self.spi.write(&[cmd])?;

        self.dc.set_high();

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), SPI::Error> {
        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf)?;

        Ok(())
    }
}
