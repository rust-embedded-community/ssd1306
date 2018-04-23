//! SSD1306 SPI interface

use hal;
use hal::digital::OutputPin;

use super::DisplayInterface;

// TODO: Add to prelude
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
    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), ()> {
        self.dc.set_low();

        self.spi.write(&cmds).map_err(|_| ())?;

        self.dc.set_high();

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), ()> {
        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf).map_err(|_| ())?;

        Ok(())
    }
}
