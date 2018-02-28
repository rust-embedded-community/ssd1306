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

mod interface;
use interface::{ I2cInterface, SpiInterface };

pub mod builder;

pub struct SSD1306I2C<I2C> {
    iface: I2cInterface<I2C>,
    buffer: [u8; 1024],
}

pub struct SSD1306SPI<SPI, RST, DC> {
    iface: SpiInterface<SPI, RST, DC>,
    buffer: [u8; 1024],
}

impl<SPI, RST, DC> SSD1306SPI<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
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

        let dx = endx as i32 - startx as i32;
        let mut dy = endy as i32 - starty as i32;

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

        let mut dx = endx as i32 - startx as i32;
        let dy = endy as i32 - starty as i32;

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
}

impl<I2C> SSD1306I2C<I2C> where I2C: hal::blocking::i2c::Write {
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

        let dx = endx as i32 - startx as i32;
        let mut dy = endy as i32 - starty as i32;

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

        let mut dx = endx as i32 - startx as i32;
        let dy = endy as i32 - starty as i32;

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
}

impl<I2C> Drawing for SSD1306I2C<I2C> where I2C: hal::blocking::i2c::Write {
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

    // [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
    fn line(&mut self, start: (u32, u32), end: (u32, u32), value: u8) {
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

    fn rect(&mut self, tl: (u32, u32), br: (u32, u32), value: u8) {
        // Top
        self.line((tl.0, tl.1), (br.0, tl.1), value);

        // Right
        self.line((br.0, tl.1), (br.0, br.1), value);

        // Bottom
        self.line((br.0, br.1), (tl.0, br.1), value);

        // Left
        self.line((tl.0, tl.1), (tl.0, br.1), value);
    }

    // [Midpoint circle algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm)
    fn center_circle(&mut self, center: (u32, u32), radius: u32, value: u8) {
        let x0 = center.0 as i32;
        let y0 = center.1 as i32;

        let rad = radius as i32 + 1;

        let mut x: i32 = rad - 1;
        let mut y: i32 = 0;
        let mut dx: i32 = 1;
        let mut dy: i32 = 1;
        let mut err: i32 = dx - (rad << 1);

        while x >= y {
            self.set_pixel((x0 + x) as u32, (y0 + y) as u32, value);
            self.set_pixel((x0 + y) as u32, (y0 + x) as u32, value);
            self.set_pixel((x0 - y) as u32, (y0 + x) as u32, value);
            self.set_pixel((x0 - x) as u32, (y0 + y) as u32, value);
            self.set_pixel((x0 - x) as u32, (y0 - y) as u32, value);
            self.set_pixel((x0 - y) as u32, (y0 - x) as u32, value);
            self.set_pixel((x0 + y) as u32, (y0 - x) as u32, value);
            self.set_pixel((x0 + x) as u32, (y0 - y) as u32, value);

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            } if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - (rad << 1);
            }
        }
    }
}

impl<SPI, RST, DC> Drawing for SSD1306SPI<SPI, RST, DC> where
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

    // [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
    fn line(&mut self, start: (u32, u32), end: (u32, u32), value: u8) {
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

    fn rect(&mut self, tl: (u32, u32), br: (u32, u32), value: u8) {
        // Top
        self.line((tl.0, tl.1), (br.0, tl.1), value);

        // Right
        self.line((br.0, tl.1), (br.0, br.1), value);

        // Bottom
        self.line((br.0, br.1), (tl.0, br.1), value);

        // Left
        self.line((tl.0, tl.1), (tl.0, br.1), value);
    }

    // [Midpoint circle algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm)
    fn center_circle(&mut self, center: (u32, u32), radius: u32, value: u8) {
        let x0 = center.0 as i32;
        let y0 = center.1 as i32;

        let rad = radius as i32 + 1;

        let mut x: i32 = rad - 1;
        let mut y: i32 = 0;
        let mut dx: i32 = 1;
        let mut dy: i32 = 1;
        let mut err: i32 = dx - (rad << 1);

        while x >= y {
            self.set_pixel((x0 + x) as u32, (y0 + y) as u32, value);
            self.set_pixel((x0 + y) as u32, (y0 + x) as u32, value);
            self.set_pixel((x0 - y) as u32, (y0 + x) as u32, value);
            self.set_pixel((x0 - x) as u32, (y0 + y) as u32, value);
            self.set_pixel((x0 - x) as u32, (y0 - y) as u32, value);
            self.set_pixel((x0 - y) as u32, (y0 - x) as u32, value);
            self.set_pixel((x0 + y) as u32, (y0 - x) as u32, value);
            self.set_pixel((x0 + x) as u32, (y0 - y) as u32, value);

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            } if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - (rad << 1);
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
