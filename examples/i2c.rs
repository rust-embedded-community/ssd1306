//! Draw a square, circle and triangle on the screen using I2C1 on pins B9 and B9
//!
//! Requires pullup resistors on SDA and SCL to function correctly

#![no_std]

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate stm32f103xx_hal as blue_pill;

extern crate ssd1306;

use blue_pill::prelude::*;
use blue_pill::i2c::{DutyCycle, I2c, Mode};

use ssd1306::{Builder, Drawing};

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

    let mut disp = Builder::new().connect_i2c(i2c);

    // Frame
    disp.rect((0, 0), (127, 63), 1u8);

    // Draw a triangle
    disp.line((8, 16 + 16), (8 + 16, 16 + 16), 1u8); // Bottom
    disp.line((8, 16 + 16), (8 + 8, 16), 1u8); // Left
    disp.line((8 + 16, 16 + 16), (8 + 8, 16), 1u8); // Right

    // Draw a square
    disp.rect((48, 16), (48 + 16, 16 + 16), 1u8);

    // Draw a circle
    disp.center_circle((96, 16 + 8), 8, 1u8);

    disp.flush();
}
