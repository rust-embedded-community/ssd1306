use hal;

pub struct I2cInterface<I2C> {
	i2c: I2C
}

impl<I2C> I2cInterface<I2C> where I2C: hal::blocking::i2c::Write {
	pub fn new(i2c: I2C) -> Self {
		let mut iface = Self { i2c };

		iface.init();

		iface
	}

	fn cmds(&mut self, cmds: &[u8]) {
	    for c in cmds {
	        self.i2c.write(0x3c, &[ 0, *c ]);
	    }
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

	    // Data mode
	    // 8.1.5.2 5) b) in the datasheet
	    // TODO: Build one buffer and send as one `i2c.write()` call. The code below is slow as balls right now
	    for byte in buf.iter() {
	        self.i2c.write(0x3c, &[ 0x40, *byte ]);
	    }
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

	    self.cmds(&init_commands);
	}
}