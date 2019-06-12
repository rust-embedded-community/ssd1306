//! SSD1306 SPI interface

use hal;
use hal::digital::v2::OutputPin;

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

impl<SPI, DC, CommE, PinE> SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    /// Create new SPI interface for communciation with SSD1306
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }
}

impl<SPI, DC, CommE, PinE> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.dc.set_low().map_err(Error::Pin)?;

        self.spi.write(&cmds).map_err(Error::Comm)?;

        self.dc.set_high().map_err(Error::Pin)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // 1 = data, 0 = command
        self.dc.set_high().map_err(Error::Pin)?;

        self.spi.write(&buf).map_err(Error::Comm)
    }
}
