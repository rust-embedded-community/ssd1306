//! SSD1306 SPI interface

use hal::{self, digital::v2::OutputPin};

use super::DisplayInterface;
use crate::Error;
use display_interface::WriteOnlyDataCommand;
use display_interface_spi::SPIInterfaceNoCS as DSPIInterface;

// TODO: Add to prelude
/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC> {
    inner: DSPIInterface<SPI, DC>,
}

impl<SPI, DC, CommE, PinE> SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    /// Create new SPI interface for communciation with SSD1306
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self {
            inner: DSPIInterface::new(spi, dc),
        }
    }
}

impl<SPI, DC, CommE, PinE> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.inner.send_commands(cmds).map_err(Error::Comm)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.inner.send_data(buf).map_err(Error::Comm)
    }
}
