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
//! PA5 -> SCL
//! PA7 -> SDA
//! PB0 -> RST
//! PB1 -> D/C
//! ```
//!
//! Run on a Blue Pill with `cargo run --example pixelsquare`.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::stm32;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

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

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
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

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();

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

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
