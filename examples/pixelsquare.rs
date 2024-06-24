//! This example draws a small square one pixel at a time in the top left corner of the display
//!
//! You will probably want to use the [`embedded_graphics`](https://crates.io/crates/embedded-graphics) crate to do more complex drawing.
//!
//! This example is for the STM32F103 "Blue Pill" board using a 4 wire interface to the display on
//! SPI1.
//!
//! Wiring connections are as follows
//!
//! ```
//! GND -> GND
//! 3V3 -> VCC
//! PA5 -> SCL (D0)
//! PA7 -> SDA (D1)
//! PB0 -> RST
//! PB1 -> D/C
//! PB10 -> CS
//! ```
//!
//! Run on a Blue Pill with `cargo run --example pixelsquare`.

#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_stm32::{
    gpio,
    spi::{self, Spi},
    time::Hertz,
};
use panic_probe as _;
use ssd1306::{prelude::*, Ssd1306};

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut config = spi::Config::default();
    config.frequency = Hertz::mhz(8);
    let spi = Spi::new_blocking_txonly(p.SPI1, p.PA5, p.PA7, config);

    let mut rst = gpio::Output::new(p.PB0, gpio::Level::Low, gpio::Speed::Low);
    let dc = gpio::Output::new(p.PB1, gpio::Level::Low, gpio::Speed::Low);
    let cs = gpio::Output::new(p.PB10, gpio::Level::Low, gpio::Speed::Low);
    let spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let interface = display_interface_spi::SPIInterface::new(spi, dc);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .reset(&mut rst, &mut embassy_time::Delay {})
        .unwrap();
    display.init().unwrap();

    // Top side
    display.set_pixel(0, 0, true);
    display.set_pixel(1, 0, true);
    display.set_pixel(2, 0, true);
    display.set_pixel(3, 0, true);

    // Right side
    display.set_pixel(3, 0, true);
    display.set_pixel(3, 1, true);
    display.set_pixel(3, 2, true);
    display.set_pixel(3, 3, true);

    // Bottom side
    display.set_pixel(0, 3, true);
    display.set_pixel(1, 3, true);
    display.set_pixel(2, 3, true);
    display.set_pixel(3, 3, true);

    // Left side
    display.set_pixel(0, 0, true);
    display.set_pixel(0, 1, true);
    display.set_pixel(0, 2, true);
    display.set_pixel(0, 3, true);

    display.flush().unwrap();

    loop {
        nop()
    }
}
