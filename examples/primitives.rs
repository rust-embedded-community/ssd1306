//! Draw a square, circle and triangle on the screen

#![no_std]

extern crate cortex_m;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

extern crate ssd1306;

use blue_pill::prelude::*;
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };

use ssd1306::{ SSD1306SPI, Drawing };

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

    let mut disp = SSD1306SPI::new(spi, rst, dc);

    // Frame
    disp.rect((0, 0), (127, 63), 1u8);

    // Draw a triangle
    disp.line((8, 16 + 16), (8 + 16, 16 + 16), 1u8);    // Bottom
    disp.line((8, 16 + 16), (8 + 8, 16), 1u8);          // Left
    disp.line((8 + 16, 16 + 16), (8 + 8, 16), 1u8);     // Right

    // Draw a square
    disp.rect((48, 16), (48 + 16, 16 + 16), 1u8);

    // Draw a circle
    disp.center_circle((96, 16 + 8), 8, 1u8);

    disp.flush();
}
