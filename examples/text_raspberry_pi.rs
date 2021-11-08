//! Print "Hello world!" and on a 128x64 display
//! connected to the i2c bus of a Raspberry Pi. Before running this example
//! make sure the i2c interface is enabled by configuring the Raspberry Pi:
//!
//! ```
//! sudo raspi-config
//! ```
//!
//! Select `3. Interface options` and enable the i2c interface. Reboot
//! the system with `sudo reboot`
//!
//!
//! Wiring the display to the Raspberry pi 4:
//!
//! ```
//! Pin 1 -> 3.3v power
//! Pin 3 -> GPIO 2 (SDA)
//! Pin 5 -> GPIO 3 (SCL)
//! Pin 6 -> Ground
//! ```
//!
//! Run this example on a raspberry pi: `cargo run --example text_raspberry_pi`
//!
//! If you want to build this example on a different system, you first have to install
//! the GNU C compiler for the armhf architecture by running
//! `sudo apt install gcc-arm-linux-gnueabihf`
//!
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();
}
