//! Endlessly fill the screen with characters from the alphabet
//!
//! Run on a Blue Pill with `xargo run --example character_i2c

#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{DutyCycle, I2c, Mode};
use blue_pill::prelude::*;
use ssd1306::{Builder, mode::CharacterMode};
use core::fmt::Write;

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

    let mut disp: CharacterMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.clear();

    /* Endless loop */
    loop {
        for c in 97..123 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
        for c in 65..91 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
    }
}
