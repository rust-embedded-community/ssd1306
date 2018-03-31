//! Graphical OLED version of Hello World. Blinks two 4px wide squares on and off in the top left
//! corner of the display.
//!
//! This example is for the STM32F103 "Blue Pill" board using a 4 wire interface to the display on
//! SPI1.
//!
//! Wiring connections are as follows
//!
//! ```
//! GND -> GND
//! 3V3 -> VCC
//! PA5 -> SCL
//! PA7 -> SDA
//! PB0 -> RST
//! PB1 -> D/C
//! ```
//!
//! Run on a Blue Pill with `xargo run --example blinky --features graphics`

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

use blue_pill::delay::Delay;
use blue_pill::gpio::gpioa::{PA5, PA6, PA7};
use blue_pill::gpio::gpiob::PB1;
use blue_pill::gpio::{Alternate, Floating, Input, Output, PushPull};
use blue_pill::prelude::*;
use blue_pill::spi::Spi;
use blue_pill::stm32f103xx::SPI1;
use cortex_m_rtfm_macros::app;
use hal::spi::{Mode, Phase, Polarity};
use rtfm::Threshold;
use ssd1306::interface::SpiInterface;
use ssd1306::{Builder, mode::GraphicsMode};

pub type OledDisplay = GraphicsMode<
    SpiInterface<
        Spi<
            SPI1,
            (
                PA5<Alternate<PushPull>>,
                PA6<Input<Floating>>,
                PA7<Alternate<PushPull>>,
            ),
        >,
        PB1<Output<PushPull>>, // B1 -> DC
    >,
>;

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

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let mut delay = Delay::new(p.core.SYST, clocks);

    // SPI1
    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp = Builder::new().connect_spi(spi, dc).into_graphicsmode();

    disp.reset(&mut rst, &mut delay);
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
