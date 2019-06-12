//! SSD1306 SPI interface

use hal;
use hal::digital::OutputPin;

use super::DisplayInterface;
use crate::Error;

// TODO: Add to prelude
/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC, CommE> SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin,
{
    /// Create new SPI interface for communciation with SSD1306
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC, CommE> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin,
{
    type Error = Error<CommE>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.dc.set_low();

        self.spi.write(&cmds).map_err(Error::Comm)?;

        self.dc.set_high();

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // 1 = data, 0 = command
        self.dc.set_high();


        self.spi.write(&buf).map_err(Error::Comm)
    }
}
