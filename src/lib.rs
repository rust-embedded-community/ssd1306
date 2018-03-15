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

        disp.init();
        disp.flush();

        disp
    }

    pub fn flush(&mut self) {
        self.iface.send_data(&self.buffer);
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

    // Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) {
        let init_commands: [u8; 25] = [
            0xAE,       // 0 disp off
            0xD5,       // 1 clk div
            0x80,       // 2 suggested ratio
            0xA8, 63,   // 3 set multiplex, height-1
            0xD3, 0x0,  // 5 display offset
            0x40,       // 7 start line
            0x8D, 0x14, // 8 charge pump
            0x20, 0x00, // 10 memory mode, 0x20 = address mode command, 0x00 = horizontal address mode
            0xA1,       // 12 seg remap 1
            0xC8,       // 13 comscandec
            0xDA, 0x12, // 14 set compins, height==64 ? 0x12:0x02,
            0x81, 0xCF, // 16 set contrast
            0xD9, 0xF1, // 18 set precharge
            0xDb, 0x40, // 20 set vcom detect
            0xA4,       // 22 display all on
            0xA6,       // 23 display normal (non-inverted)
            0xAf        // 24 disp on
        ];

        for c in init_commands.iter() {
            self.iface.send_command(*c);
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
