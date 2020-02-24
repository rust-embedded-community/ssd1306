//! Draw an RGB565 BMP image onto the display by converting the `Rgb565` pixel color type to
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

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
};
use panic_halt as _;
use ssd1306::{prelude::*, Builder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};
use tinybmp::Bmp;

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
            frequency: 400_000.hz(),
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

    let bmp =
        Bmp::from_slice(include_bytes!("./rust-pride.bmp")).expect("Failed to load BMP image");

    // The image is an RGB565 encoded BMP, so specifying the type as `Image<Bmp, Rgb565>` will read
    // the pixels correctly
    let im: Image<Bmp, Rgb565> = Image::new(&bmp, Point::new(32, 0));

    // The display uses `BinaryColor` pixels (on/off only). Here, we `map()` over every pixel
    // and naively convert the color to an on/off value. The logic below simply converts any
    // color that's not black into an "on" pixel.
    im.into_iter()
        .map(|Pixel(position, color)| {
            Pixel(
                position,
                if color != Rgb565::BLACK {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                },
            )
        })
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
