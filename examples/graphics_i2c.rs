//! Draw a square, circle and triangle on the screen using the `embedded_graphics` crate.
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
//! Run on a Blue Pill with `cargo run --example graphics_i2c`, currently only works on nightly.

#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate panic_abort;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{DutyCycle, I2c, Mode};
use blue_pill::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use ssd1306::prelude::*;
use ssd1306::Builder;

fn main() {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(Line::new((8, 16 + 16), (8 + 16, 16 + 16), 1).into_iter());
    disp.draw(Line::new((8, 16 + 16), (8 + 8, 16), 1).into_iter());
    disp.draw(Line::new((8 + 16, 16 + 16), (8 + 8, 16), 1).into_iter());

    disp.draw(Rect::new((48, 16), (48 + 16, 16 + 16), 1u8).into_iter());

    disp.draw(Circle::new((96, 16 + 8), 8, 1u8).into_iter());

    disp.flush().unwrap();
}
