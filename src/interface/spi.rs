//! SSD1306 SPI interface

use hal::{self, digital::v2::OutputPin};

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

    fn send_bounded_data(
        &mut self,
        buf: &[u8],
        disp_width: usize,
        upper_left: (u8, u8),
        lower_right: (u8, u8),
    ) -> Result<(), Self::Error> {
        self.dc.set_high().map_err(Error::Pin)?;

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
            .try_for_each(|c| self.spi.write(&c).map_err(Error::Comm))
    }
}
