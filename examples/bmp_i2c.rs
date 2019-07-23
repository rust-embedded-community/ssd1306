//! Draw an RGB565 BMP image onto the display by converting the `Rgb565` pixel colour type to
//! `BinaryColor` using a simple threshold where any pixel with a value greater than zero is treated
//! as "on".
//!
//! Note that the `bmp` feature for `embedded-graphics` must be turned on.
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
//! Run on a Blue Pill with `cargo run --example image_i2c`.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::image::ImageBmp;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use hal::i2c::{BlockingI2c, DutyCycle, Mode};
use hal::prelude::*;
use hal::stm32;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

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

    // The image is an RGB565 encoded BMP, so specifying the type as `ImageBmp<Rgb565>` will read
    // the pixels correctly
    let im: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!("./rust-pride.bmp"))
        .expect("Failed to load BMP image")
        .translate(Coord::new(32, 0));

    // The display uses `BinaryColor` pixels (on/off only). Here, we `map()` over every pixel
    // and naively convert the colour to an on/off value. The logic below simply converts any
    // colour that's not black into an "on" pixel.
    let im = im.into_iter().map(|Pixel(position, colour)| {
        let grey = (colour.r() as u16 + colour.g() as u16 + colour.b() as u16) / 3;

        Pixel(
            position,
            if grey > 0 {
                BinaryColor::On
            } else {
                BinaryColor::Off
            },
        )
    });

    disp.draw(im);

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
