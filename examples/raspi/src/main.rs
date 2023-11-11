use embedded_graphics::{
    fonts::{Font6x8, Text},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::{PrimitiveStyleBuilder, TextStyleBuilder},
    DrawTarget,
};
use linux_embedded_hal::I2cdev;
use machine_ip;
use ssd1306::{mode::GraphicsMode, Builder, I2CDIBuilder};
use std::thread::sleep;
use std::time::Duration;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    while running.load(Ordering::SeqCst) {
        Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
            .into_styled(style)
            .into_iter()
            .draw(&mut disp);
        Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
            .into_styled(style)
            .into_iter()
            .draw(&mut disp);
        Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
            .into_styled(style)
            .into_iter()
            .draw(&mut disp);
        Rectangle::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
            .into_styled(style)
            .into_iter()
            .draw(&mut disp);
        Circle::new(Point::new(96, 16 + 8), 8)
            .into_styled(style)
            .into_iter()
            .draw(&mut disp);

        let local_addr = machine_ip::get().unwrap();

        Text::new(
            &format!("IP: {}", local_addr.to_string()),
            Point::new(0, 56),
        )
        .into_styled(text_style)
        .into_iter()
        .draw(&mut disp);

        disp.flush().unwrap();

        sleep(Duration::from_secs(2));

        disp.clear();

        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);

        let im = Image::new(&raw, Point::new(32, 0));
        im.draw(&mut disp).unwrap();
        disp.flush().unwrap();

        sleep(Duration::from_secs(2));
        disp.clear();
    }
    disp.clear();
    disp.flush().unwrap();
}
