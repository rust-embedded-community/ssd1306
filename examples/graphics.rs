//! Draw a square, circle and triangle on the screen using the embedded_graphics library over a 4
//! wire SPI interface.
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
//! Run on a Blue Pill with `cargo run --example graphics`.

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
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
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

    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Size::new(127, 63))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    // square
    Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // circle
    Circle::new(Point::new(88, yoffset), 16)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        nop()
    }
}
