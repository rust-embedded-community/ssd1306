#![no_std]

extern crate embedded_hal as hal;

use hal::digital::OutputPin;

pub struct SSD1306<SPI, RST, DC>
{
    spi: SPI,
    rst: RST,
    dc: DC,
    buffer: [u8; 1024],
}

// Currently only implemented for 4 wire SPI, 128x63 monocrhrome OLED
impl<SPI, RST, DC> SSD1306<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
        SSD1306 {
            spi,
            rst,
            dc,
            buffer: [0b10101010; 1024],
        }
    }

    pub fn reset(&mut self) {
        self.rst.set_low();
        self.rst.set_high();
    }

    pub fn cmd(&mut self, cmd: u8) {
       self.cmds(&[ cmd ]);
    }

    pub fn cmds(&mut self, cmds: &[u8]) {
        self.dc.set_low();

        self.spi.write(cmds);

        self.dc.set_high();
    }

    pub fn init(&mut self) {
        let init_commands: [ u8; 25 ] = [
            0xAE, // 0 disp off
            0xD5, // 1 clk div
            0x80, // 2 suggested ratio
            0xA8, 63, // 3 set multiplex, height-1
            0xD3, 0x0, // 5 display offset
            0x40, // 7 start line
            0x8D, 0x14, // 8 charge pump
            0x20, 0x0, // 10 memory mode
            0xA1, // 12 seg remap 1
            0xC8, // 13 comscandec
            0xDA, 0x12, // 14 set compins, height==64 ? 0x12:0x02,
            0x81, 0xCF, // 16 set contrast
            0xD9, 0xF1, // 18 set precharge
            0xDb, 0x40, // 20 set vcom detect
            0xA4, // 22 display all on
            0xA6, // 23 display normal (non-inverted)
            0xAf // 24 disp on
        ];

        self.cmds(&init_commands);
    }

    pub fn flush(&mut self) {
        let flush_commands: [ u8; 6 ] = [
             0x21, // columns
             0, 127,
             0x22, // pages
             0, 7 /* (height>>3)-1 */];

        self.cmds(&flush_commands);

        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&self.buffer);
    }
}