use hal;
use hal::digital::OutputPin;

use super::{ SSD1306SPI, SSD1306I2C };

#[derive(Clone, Copy)]
pub struct Builder { }

impl Builder {
	pub fn new() -> Self {
		Self {}
	}

	pub fn connect_i2c<I2C>(&self, i2c: I2C) -> SSD1306I2C<I2C> where I2C: hal::blocking::i2c::Write {
		SSD1306I2C::new(i2c)
	}

	pub fn connect_spi<SPI, RST, DC>(&self, spi: SPI, rst: RST, dc: DC) -> SSD1306SPI<SPI, RST, DC> where
	    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
	    RST: OutputPin,
	    DC: OutputPin
	    {
		SSD1306SPI::new(spi, rst, dc)
	}
}