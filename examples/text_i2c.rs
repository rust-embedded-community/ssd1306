//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example text_i2c`, currently only works on nightly.

#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
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

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(
        Font6x8::render_str("Hello world!")
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    disp.draw(
        Font6x8::render_str("Hello Rust!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(0, 16))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
