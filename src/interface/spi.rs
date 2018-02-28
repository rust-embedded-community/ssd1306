use hal;
use hal::digital::OutputPin;

pub struct SpiInterface<SPI, RST, DC> {
	spi: SPI,
	rst: RST,
	dc: DC,
}

impl<SPI, RST, DC> SpiInterface<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
		let mut iface = Self { spi, rst, dc };

		iface.reset();
		iface.init();

		iface
	}

	fn cmds(&mut self, cmds: &[u8]) {
	    self.dc.set_low();

	    self.spi.write(cmds);

	    self.dc.set_high();
	}

	pub fn flush(&mut self, buf: &[u8]) {
        let flush_commands: [ u8; 6 ] = [
            0x21, // Set column address from addr...
            0,    // 0 to ...
            127,  // 128 columns (0 indexed).

            0x22, // Set pages from addr ...
            0,    // 0 to ...
            7     // 8 pages (0 indexed). 8 pages of 8 rows (1 byte) each = 64px high
        ];

        self.cmds(&flush_commands);

        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf);
    }


	// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
	fn init(&mut self) {
	    let init_commands: [ u8; 25 ] = [
	        0xAE,       // 0 disp off
	        0xD5,       // 1 clk div
	        0x80,       // 2 suggested ratio
	        0xA8, 63,   // 3 set multiplex, height-1
	        0xD3, 0x0,  // 5 display offset
	        0x40,       // 7 start line
	        0x8D, 0x14, // 8 charge pump
	        0x20, 0x00, // 10 memory mode, 0x20 = address mode command, 0x00 = horizontal address mode
	        0xA1,       // 12 seg remap 1
	        0xC8,       // 13 comscandec
	        0xDA, 0x12, // 14 set compins, height==64 ? 0x12:0x02,
	        0x81, 0xCF, // 16 set contrast
	        0xD9, 0xF1, // 18 set precharge
	        0xDb, 0x40, // 20 set vcom detect
	        0xA4,       // 22 display all on
	        0xA6,       // 23 display normal (non-inverted)
	        0xAf        // 24 disp on
	    ];

	    self.reset();

	    self.cmds(&init_commands);
	}

	fn reset(&mut self) {
	    self.rst.set_low();
	    self.rst.set_high();
	}
}