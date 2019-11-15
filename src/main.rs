use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use embedded_graphics::Drawing;
use linux_embedded_hal::I2cdev;
use machine_ip;
use ssd1306::{mode::GraphicsMode, Builder};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16)).into_iter());
    disp.draw(Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16)).into_iter());
    disp.draw(Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16)).into_iter());

    disp.draw(Rectangle::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16)).into_iter());

    disp.draw(Circle::new(Point::new(96, 16 + 8), 8).into_iter());

    let local_addr = machine_ip::get().unwrap();

    disp.draw(
        Font6x8::render_str(&format!("IP: {}", local_addr.to_string()))
            .translate(Point::new(0, 56))
            .into_iter(),
    );

    disp.flush().unwrap();
}
