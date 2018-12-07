//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Run on a Blue Pill with `cargo run --example graphics_i2c_128x32`, currently only works on
//! nightly.

#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use ssd1306::prelude::*;
use ssd1306::Builder;
use stm32f103xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f103xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = stm32f103xx_hal::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let yoffset = 8;

    disp.draw(
        Line::new(
            Coord::new(8, 16 + yoffset),
            Coord::new(8 + 16, 16 + yoffset),
        )
        .with_stroke(Some(1u8.into()))
        .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8 + 16, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Rect::new(Coord::new(48, yoffset), Coord::new(48 + 16, 16 + yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Circle::new(Coord::new(96, yoffset + 8), 8)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
