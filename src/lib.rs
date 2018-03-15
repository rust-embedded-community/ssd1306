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

mod interface;
use interface::DisplayInterface;

pub mod builder;
pub use builder::Builder;

pub struct SSD1306<DI> {
    iface: DI,
    buffer: [u8; 1024],
}

impl<DI> SSD1306<DI>
where
    DI: DisplayInterface,
{
    pub fn new(iface: DI) -> SSD1306<DI> {
        let mut disp = SSD1306 {
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

impl<DI> Drawing for SSD1306<DI>
where
    DI: DisplayInterface,
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
