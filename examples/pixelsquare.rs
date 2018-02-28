//! This example draws a small square one pixel at a time in the top left corner of the display
//!
//! You will probably want to use the [`embedded_graphics`](https://crates.io/crates/embedded-graphics) crate to do more complex drawing.

#![no_std]

extern crate cortex_m;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

extern crate ssd1306;

use blue_pill::prelude::*;
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };
use ssd1306::Builder;

fn main() {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp = Builder::new().connect_spi(spi, rst, dc);

    // Top side
    disp.set_pixel(0, 0, 1);
    disp.set_pixel(1, 0, 1);
    disp.set_pixel(2, 0, 1);
    disp.set_pixel(3, 0, 1);

    // Right side
    disp.set_pixel(3, 0, 1);
    disp.set_pixel(3, 1, 1);
    disp.set_pixel(3, 2, 1);
    disp.set_pixel(3, 3, 1);

    // Bottom side
    disp.set_pixel(0, 3, 1);
    disp.set_pixel(1, 3, 1);
    disp.set_pixel(2, 3, 1);
    disp.set_pixel(3, 3, 1);

    // Left side
    disp.set_pixel(0, 0, 1);
    disp.set_pixel(0, 1, 1);
    disp.set_pixel(0, 2, 1);
    disp.set_pixel(0, 3, 1);

    disp.flush();
}
