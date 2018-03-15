use hal;
use hal::digital::OutputPin;

use super::interface::{I2cInterface, SpiInterface};
use super::SSD1306;

#[derive(Clone, Copy)]
pub enum DisplaySize {
    Display128x64,
}

#[derive(Clone, Copy)]
pub struct Builder {
    size: DisplaySize,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            size: DisplaySize::Display128x64,
        }
    }

    pub fn with_size(&self, size: DisplaySize) -> Self {
        Self { size }
    }

    pub fn connect_i2c<I2C>(&self, i2c: I2C) -> SSD1306<I2cInterface<I2C>>
    where
        I2C: hal::blocking::i2c::Write,
    {
        SSD1306::new(I2cInterface::new(i2c))
    }

    pub fn connect_spi<SPI, DC>(&self, spi: SPI, dc: DC) -> SSD1306<SpiInterface<SPI, DC>>
    where
        SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
        DC: OutputPin,
    {
        SSD1306::new(SpiInterface::new(spi, dc))
    }
}
