//! Draw a square, circle and triangle on the screen

#![no_std]

extern crate cortex_m;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

extern crate ssd1306;

use blue_pill::prelude::*;
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };

use ssd1306::embedded_graphics::primitives;
use ssd1306::{ SSD1306SPI, Drawing, Builder };

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

    let tri1 = primitives::Line { start: (8, 16 + 16), end: (8 + 16, 16 + 16), color: 1 };
    let tri2 = primitives::Line { start: (8, 16 + 16), end: (8 + 8, 16), color: 1 };
    let tri3 = primitives::Line { start: (8 + 16, 16 + 16), end: (8 + 8, 16), color: 1 };

    disp.draw(tri1.into_iter());
    disp.draw(tri2.into_iter());
    disp.draw(tri3.into_iter());

    disp.draw(primitives::Rect { top_left: (48, 16), bottom_right: (48 + 16, 16 + 16), color: 1u8 }.into_iter());

    disp.draw(primitives::Circle { center: (96, 16 + 8), radius: 8, color: 1u8 }.into_iter());

    disp.flush();
}
