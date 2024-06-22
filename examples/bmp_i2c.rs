//! Draw an RGB565 BMP image onto the display by converting the `Rgb565` pixel color type to
//! `BinaryColor` using a simple threshold where any pixel with a value greater than zero is treated
//! as "on".
//!
//! Note that the `bmp` feature for `embedded-graphics` must be turned on.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB7
//! (green)  SCL -> PB6
//! ```
//!
//! Run on a Blue Pill with `cargo run --example image_i2c`.

#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use tinybmp::Bmp;

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let bmp = Bmp::from_slice(include_bytes!("./rust.bmp")).expect("Failed to load BMP image");

    // The image is an RGB565 encoded BMP, so specifying the type as `Image<Bmp<Rgb565>>` will read
    // the pixels correctly
    let im: Image<Bmp<Rgb565>> = Image::new(&bmp, Point::new(32, 0));

    // We use the `color_converted` method here to automatically convert the RGB565 image data into
    // BinaryColor values.
    im.draw(&mut display.color_converted()).unwrap();

    display.flush().unwrap();

    loop {
        nop()
    }
}
