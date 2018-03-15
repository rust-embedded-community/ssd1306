#![no_std]
// TODO: Docs
// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate embedded_graphics;
extern crate embedded_hal as hal;

use embedded_graphics::Drawing;
use embedded_graphics::drawable;
use hal::digital::OutputPin;
use num_traits::*;

mod interface;
use interface::{I2cInterface, SpiInterface};

pub mod builder;
pub use builder::Builder;

pub struct SSD1306I2C<I2C> {
    iface: I2cInterface<I2C>,
    buffer: [u8; 1024],
}

pub struct SSD1306SPI<SPI, RST, DC> {
    iface: SpiInterface<SPI, RST, DC>,
    buffer: [u8; 1024],
}

impl<SPI, RST, DC> SSD1306SPI<SPI, RST, DC>
where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin,
{
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
        let iface = SpiInterface::new(spi, rst, dc);
        let mut disp = SSD1306SPI {
            iface,
            buffer: [0; 1024],
        };

        disp.iface.flush(&disp.buffer);

        disp
    }

    pub fn flush(&mut self) {
        self.iface.flush(&self.buffer);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let byte = &mut self.buffer[((y as usize) / 8 * 128) + (x as usize)];
        let bit = 1 << (y % 8);

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }
}

impl<SPI, RST, DC> Drawing for SSD1306SPI<SPI, RST, DC>
where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel>,
    {
        for (pos, color) in item_pixels {
            self.set_pixel(pos.0, pos.1, color);
        }
    }
}

impl<I2C> SSD1306I2C<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    pub fn new(i2c: I2C) -> Self {
        let iface = I2cInterface::new(i2c);
        let mut disp = SSD1306I2C {
            iface,
            buffer: [0; 1024],
        };

        disp.iface.flush(&disp.buffer);

        disp
    }

    pub fn flush(&mut self) {
        self.iface.flush(&self.buffer);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let byte = &mut self.buffer[((y as usize) / 8 * 128) + (x as usize)];
        let bit = 1 << (y % 8);

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }
}

impl<I2C> Drawing for SSD1306I2C<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel>,
    {
        for (pos, color) in item_pixels {
            self.set_pixel(pos.0, pos.1, color);
        }
    }
}
