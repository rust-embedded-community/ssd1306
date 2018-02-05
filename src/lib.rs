#![no_std]

extern crate embedded_hal as hal;
extern crate embedded_graphics;

use hal::digital::OutputPin;
pub use embedded_graphics::Drawing;

pub struct SSD1306<SPI, RST, DC>
{
    spi: SPI,
    rst: RST,
    dc: DC,
    buffer: [u8; 1024],
}

// Currently only implemented for 4 wire SPI, 128x63 monocrhrome OLED
impl<SPI, RST, DC> SSD1306<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
        SSD1306 {
            spi,
            rst,
            dc,
            buffer: [0; 1024],
        }
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
}

impl<SPI, RST, DC> Drawing for SSD1306<SPI, RST, DC> {
    fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (byte_offset, bit_offset) = coords_to_index(x, y);

        if value == 0 {
            self.buffer[byte_offset] &= !(1 << bit_offset);
        } else {
            self.buffer[byte_offset] |= 1 << bit_offset;
        }
    }

    fn draw_image_8bpp(&mut self, bytes: &[u8], w: u32, h: u32) {
        for x in 0..w {
            for y in 0..h {
                let offset = (y * w) + x;

                self.set_pixel(x, y, bytes[offset as usize]);
            }
        }
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
        fn it_sets_a_pixel_at_63x63() {
            assert_eq!(coords_to_index(63, 63), (959, 7));
        }
    }
}
