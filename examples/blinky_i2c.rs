//! Graphical OLED version of Hello World. Blinks two 4px wide squares on and off in the top left
//! corner of the display.
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
//! Run on a Blue Pill with `xargo run --example blinky_i2c`, currently only works on nightly.

#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate embedded_hal as hal;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::gpio::gpiob::{PB8, PB9};
use blue_pill::gpio::{Alternate, OpenDrain};
use blue_pill::prelude::*;
use blue_pill::stm32f103xx::I2C1;
use blue_pill::i2c::{DutyCycle, I2c, Mode};
use cortex_m_rtfm_macros::app;
use rtfm::Threshold;
use ssd1306::interface::I2cInterface;
use ssd1306::{Builder, mode::GraphicsMode};

pub type OledDisplay =
    GraphicsMode<I2cInterface<I2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>>;

// Tasks and resources
app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static DISP: OledDisplay;
        static STATE: bool;
    },

    idle: {
        resources: [
            DISP,
            STATE,
        ],
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        p.device.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );

    let mut disp = Builder::new().connect_i2c(i2c).into_graphicsmode();

    disp.init().unwrap();
    disp.flush().unwrap();

    init::LateResources {
        DISP: disp,
        STATE: false,
    }
}

fn draw_square(disp: &mut OledDisplay, xoffset: u32, yoffset: u32) {
    // Top side
    disp.set_pixel(xoffset + 0, yoffset + 0, 1);
    disp.set_pixel(xoffset + 1, yoffset + 0, 1);
    disp.set_pixel(xoffset + 2, yoffset + 0, 1);
    disp.set_pixel(xoffset + 3, yoffset + 0, 1);

    // Right side
    disp.set_pixel(xoffset + 3, yoffset + 0, 1);
    disp.set_pixel(xoffset + 3, yoffset + 1, 1);
    disp.set_pixel(xoffset + 3, yoffset + 2, 1);
    disp.set_pixel(xoffset + 3, yoffset + 3, 1);

    // Bottom side
    disp.set_pixel(xoffset + 0, yoffset + 3, 1);
    disp.set_pixel(xoffset + 1, yoffset + 3, 1);
    disp.set_pixel(xoffset + 2, yoffset + 3, 1);
    disp.set_pixel(xoffset + 3, yoffset + 3, 1);

    // Left side
    disp.set_pixel(xoffset + 0, yoffset + 0, 1);
    disp.set_pixel(xoffset + 0, yoffset + 1, 1);
    disp.set_pixel(xoffset + 0, yoffset + 2, 1);
    disp.set_pixel(xoffset + 0, yoffset + 3, 1);
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    let state: &'static mut bool = r.STATE;
    let mut disp: &'static mut OledDisplay = r.DISP;

    loop {
        disp.clear();

        match *state {
            true => draw_square(&mut disp, 0, 0),
            false => draw_square(&mut disp, 6, 0),
        }

        disp.flush().unwrap();

        *state = !*state;
    }
}
