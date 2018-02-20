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

extern crate num_traits;
extern crate embedded_hal as hal;
pub extern crate embedded_graphics;

pub use embedded_graphics::Drawing;
use embedded_graphics::fonts::{ Font, Font6x8 };
use embedded_graphics::image::{ Image8BPP, Image1BPP };
use hal::digital::OutputPin;
use num_traits::*;

pub struct SSD1306<SPI, RST, DC> {
    spi: SPI,
    rst: RST,
    dc: DC,
    buffer: [u8; 1024],
}

// Currently only implemented for 4 wire SPI, 128x64 monochrome OLED
impl<SPI, RST, DC> SSD1306<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
        let mut disp = SSD1306 {
            spi,
            rst,
            dc,
            buffer: [0; 1024],
        };

        disp.reset();

        disp.init();

        disp
    }

    pub fn reset(&mut self) {
        self.rst.set_low();
        self.rst.set_high();
    }

    pub fn cmd(&mut self, cmd: u8) {
       self.cmds(&[ cmd ]);
    }

    pub fn cmds(&mut self, cmds: &[u8]) {
        self.dc.set_low();

        self.spi.write(cmds);

        self.dc.set_high();
    }

    // Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) {
        let init_commands: [ u8; 25 ] = [
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

        self.cmds(&init_commands);

        self.flush();
    }

    pub fn flush(&mut self) {
        let flush_commands: [ u8; 6 ] = [
            0x21, // Set column address from addr...
            0,    // 0 to ...
            127,  // 128 columns (0 indexed).

            0x22, // Set pages from addr ...
            0,    // 0 to ...
            7     // 8 pages (0 indexed). 8 pages of 8 rows (1 byte) each = 64px high
        ];

        self.cmds(&flush_commands);

        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&self.buffer);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        // Noop if pixel is outside screen range
        if x > 127 || y > 63 {
            return;
        }

        let (byte_offset, bit_offset) = coords_to_index(x, y);

        if value == 0 {
            self.buffer[byte_offset] &= !(1 << bit_offset);
        } else {
            self.buffer[byte_offset] |= 1 << bit_offset;
        }
    }

    fn line_low(&mut self, start: (u32, u32), end: (u32, u32), value: u8) {
        let startx = start.0;
        let starty = start.1;
        let endx = end.0;
        let endy = end.1;

        let mut dx = (endx - startx) as i32;
        let mut dy: i32 = (endy - starty) as i32;

        let mut yi: i32 = 1;

        if dy < 0 {
            yi = -1;
            dy *= -1;
        }

        let mut delta = 2 * dy - dx;
        let mut y = starty as i32;

        for x in startx..(endx + 1) {
            self.set_pixel(x, y as u32, value);

            if delta > 0 {
                y += yi;
                delta -= 2 * dx;
            }

            delta += 2 * dy;
        }
    }

    fn line_high(&mut self, start: (u32, u32), end: (u32, u32), value: u8) {
        let startx = start.0;
        let starty = start.1;
        let endx = end.0;
        let endy = end.1;

        let mut dx: i32 = (endx - startx) as i32;
        let mut dy = (endy - starty) as i32;

        let mut xi: i32 = 1;

        if dx < 0 {
            xi = -1;
            dx *= -1;
        }

        let mut delta = 2 * dx - dy;
        let mut x = startx as i32;

        for y in starty..(endy + 1) {
            self.set_pixel(x as u32, y, value);

            if delta > 0 {
                x += xi;
                delta -= 2 * dy;
            }

            delta += 2 * dx;
        }
    }

    // [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
    pub fn line(&mut self, start: (u32, u32), end: (u32, u32), value: u8) {
        if (end.1 as f32 - start.1 as f32).abs() < (end.0 as f32 - start.0 as f32).abs() {
            if start.0 > end.0 {
                self.line_low(end, start, value);
            } else {
                self.line_low(start, end, value);
            }
        } else {
            if start.1 > end.1 {
                self.line_high(end, start, value);
            } else {
                self.line_high(start, end, value);
            }
        }
    }
}

impl<SPI, RST, DC> Drawing for SSD1306<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    fn draw_image_8bpp(&mut self, image: &Image8BPP, left: u32, top: u32) {
        for (x, y, value) in image.into_iter() {
            self.set_pixel(x + left, y + top, value);
        }
    }

    fn draw_image_1bpp(&mut self, image: &Image1BPP, left: u32, top: u32) {
        for (x, y, value) in image.into_iter() {
            self.set_pixel(x + left, y + top, value);
        }
    }

    fn draw_text_1bpp(&mut self, text: &str, left: u32, top: u32) {
        let (bitmap_data, bm_width, bm_height) = Font6x8::render_str(text).unwrap();

        self.draw_image_1bpp(&Image1BPP {
            width: bm_width,
            height: bm_height,
            imagedata: &bitmap_data,
        }, left, top);
    }
}

fn coords_to_index(x: u32, y: u32) -> (usize, u8) {
    let x_resolution = 128;
    // TODO: Dynamic width/height
    // let y_resolution = 64;

    let page_index = y / 8;
    let page_offset = page_index * x_resolution;

    let byte_offset = page_offset + x;
    let bit_offset = y - (page_index * 8);

    (byte_offset as usize, bit_offset as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod coords_to_index {
        use super::*;

        #[test]
        fn it_sets_0_0() {
            assert_eq!(coords_to_index(0, 0), (0, 0));
        }

        #[test]
        fn it_sets_bottom_left() {
            assert_eq!(coords_to_index(0, 63), (896, 7));
        }

        #[test]
        fn it_sets_top_right() {
            assert_eq!(coords_to_index(127, 0), (127, 0));
        }

        #[test]
        fn it_sets_bottom_right() {
            assert_eq!(coords_to_index(127, 63), (1023, 7));
        }

        #[test]
        fn it_sets_a_pixel_at_8x8() {
            assert_eq!(coords_to_index(7, 7), (7, 7));
        }

        #[test]
        fn it_sets_a_pixel_at_10x10() {
            assert_eq!(coords_to_index(9, 9), (137, 1));
        }

        #[test]
        fn it_sets_a_pixel_at_16x16() {
            // FIXME
            assert_eq!(coords_to_index(15, 15), ((128 + 15), 7));
        }

        #[test]
        fn it_sets_a_pixel_at_63x63() {
            assert_eq!(coords_to_index(63, 63), (959, 7));
        }
    }
}
