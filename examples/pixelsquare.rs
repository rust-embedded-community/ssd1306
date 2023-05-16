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

use cortex_m_rt::{entry, exception, ExceptionFrame};
use panic_halt as _;
use ssd1306::{prelude::*, Ssd1306};
use stm32f1xx_hal::{
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    stm32,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

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
        8.MHz(),
        clocks,
    );

    let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.reset(&mut rst, &mut delay).unwrap();
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

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
