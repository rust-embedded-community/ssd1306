//! Endlessly fill the screen with characters from the alphabet
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB7
//! (green)  SCL -> PB6
//! ```
//!
//! Run on a Blue Pill with `cargo run --example terminal_i2c`.

#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    let _ = display.clear();

    /* Endless loop */
    loop {
        for c in 97..123 {
            let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
        for c in 65..91 {
            let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
    }
}
