//! Display commands.

// Shamefully taken from https://github.com/EdgewaterDevelopment/rust-ssd1306

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

/// SSD1306 Commands
#[derive(Debug, Copy, Clone)]
pub enum Command {
    /// Set contrast. Higher number is higher contrast. Default = 0x7F
    Contrast(u8),
    /// Turn entire display on. If set, all pixels will
    /// be set to on, if not, the value in memory will be used.
    AllOn(bool),
    /// Invert display.
    Invert(bool),
    /// Turn display on or off.
    DisplayOn(bool),
    /// Set up horizontal scrolling.
    /// Values are scroll direction, start page, end page,
    /// and number of frames per step.
    HScrollSetup(HScrollDir, Page, Page, NFrames),
    /// Set up horizontal + vertical scrolling.
    /// Values are scroll direction, start page, end page,
    /// number of frames per step, and vertical scrolling offset.
    /// Scrolling offset may be from 0-63
    VHScrollSetup(VHScrollDir, Page, Page, NFrames, u8),
    /// Enable scrolling
    EnableScroll(bool),
    /// Setup vertical scroll area.
    /// Values are number of rows above scroll area (0-63)
    /// and number of rows of scrolling. (0-64)
    VScrollArea(u8, u8),
    /// Set the lower nibble of the column start address
    /// register for Page addressing mode, using the lower
    /// 4 bits given.
    /// This is only for page addressing mode
    LowerColStart(u8),
    /// Set the upper nibble of the column start address
    /// register for Page addressing mode, using the lower
    /// 4 bits given.
    /// This is only for page addressing mode
    UpperColStart(u8),
    /// Set the column start address register for Page addressing mode.
    /// Combines LowerColStart and UpperColStart
    /// This is only for page addressing mode
    ColStart(u8),
    /// Set addressing mode
    AddressMode(AddrMode),
    /// Setup column start and end address
    /// values range from 0-127
    /// This is only for horizontal or vertical addressing mode
    ColumnAddress(u8, u8),
    /// Setup page start and end address
    /// This is only for horizontal or vertical addressing mode
    PageAddress(Page, Page),
    /// Set GDDRAM page start address for Page addressing mode
    PageStart(Page),
    /// Set display start line from 0-63
    StartLine(u8),
    /// Reverse columns from 127-0
    SegmentRemap(bool),
    /// Set multiplex ratio from 15-63 (MUX-1)
    Multiplex(u8),
    /// Scan from COM[n-1] to COM0 (where N is mux ratio)
    ReverseComDir(bool),
    /// Set vertical shift
    DisplayOffset(u8),
    /// Setup com hardware configuration
    /// First value indicates sequential (false) or alternative (true)
    /// pin configuration. Second value disables (false) or enables (true)
    /// left/right remap.
    ComPinConfig(bool, bool),
    /// Set up display clock.
    /// First value is oscillator frequency, increasing with higher value
    /// Second value is divide ratio - 1
    DisplayClockDiv(u8, u8),
    /// Set up phase 1 and 2 of precharge period. Each value must be in the range 1 - 15.
    PreChargePeriod(u8, u8),
    /// Set Vcomh Deselect level
    VcomhDeselect(VcomhLevel),
    /// NOOP
    Noop,
    /// Enable charge pump
    ChargePump(bool),
    /// Select external or internal I REF. Only for 72 x 40 display with SSD1306B driver
    InternalIref(bool, bool),
    /// Noop that isn't even sent through the interface.
    ///
    /// Prefer this over [`Command::Noop`] if you don't need to send anything to the display.
    FastNoop,
}

impl Command {
    /// Transform command into a fixed size array of 7 u8 and the real length for sending
    fn encode(self, buffer: &mut [u8; 7]) -> &[u8] {
        match self {
            Command::Contrast(val) => {
                buffer[0] = 0x81;
                buffer[1] = val;

                &buffer[..2]
            }
            Command::AllOn(on) => {
                buffer[0] = 0xA4 | (on as u8);

                &buffer[..1]
            }
            Command::Invert(inv) => {
                buffer[0] = 0xA6 | (inv as u8);

                &buffer[..1]
            }
            Command::DisplayOn(on) => {
                buffer[0] = 0xAE | (on as u8);

                &buffer[..1]
            }
            Command::HScrollSetup(dir, start, end, rate) => {
                buffer[0] = 0x26 | (dir as u8);
                buffer[1] = 0;
                buffer[2] = start as u8;
                buffer[3] = rate as u8;
                buffer[4] = end as u8;
                buffer[5] = 0;
                buffer[6] = 0xFF;

                &buffer[..7]
            }
            Command::VHScrollSetup(dir, start, end, rate, offset) => {
                buffer[0] = 0x28 | (dir as u8);
                buffer[1] = 0;
                buffer[2] = start as u8;
                buffer[3] = rate as u8;
                buffer[4] = end as u8;
                buffer[5] = offset;

                &buffer[..6]
            }
            Command::EnableScroll(en) => {
                buffer[0] = 0x2E | (en as u8);

                &buffer[..1]
            }
            Command::VScrollArea(above, lines) => {
                buffer[0] = 0xA3;
                buffer[1] = above;
                buffer[2] = lines;

                &buffer[..3]
            }
            Command::LowerColStart(addr) => {
                buffer[0] = addr & 0x0F;

                &buffer[..1]
            }
            Command::UpperColStart(addr) => {
                buffer[0] = 0x10 | (addr & 0x0F);

                &buffer[..1]
            }
            Command::ColStart(addr) => {
                buffer[0] = addr & 0x0F;
                buffer[1] = 0x10 | (addr >> 4);

                &buffer[..2]
            }
            Command::AddressMode(mode) => {
                buffer[0] = 0x20;
                buffer[1] = mode as u8;

                &buffer[..2]
            }
            Command::ColumnAddress(start, end) => {
                buffer[0] = 0x21;
                buffer[1] = start;
                buffer[2] = end;

                &buffer[..3]
            }
            Command::PageAddress(start, end) => {
                buffer[0] = 0x22;
                buffer[1] = start as u8;
                buffer[2] = end as u8;

                &buffer[..3]
            }
            Command::PageStart(page) => {
                buffer[0] = 0xB0 | (page as u8);

                &buffer[..1]
            }
            Command::StartLine(line) => {
                buffer[0] = 0x40 | (line & 0x3F);

                &buffer[..1]
            }
            Command::SegmentRemap(remap) => {
                buffer[0] = 0xA0 | (remap as u8);

                &buffer[..1]
            }
            Command::Multiplex(ratio) => {
                buffer[0] = 0xA8;
                buffer[1] = ratio;

                &buffer[..2]
            }
            Command::ReverseComDir(rev) => {
                buffer[0] = 0xC0 | ((rev as u8) << 3);

                &buffer[..1]
            }
            Command::DisplayOffset(offset) => {
                buffer[0] = 0xD3;
                buffer[1] = offset;

                &buffer[..2]
            }
            Command::ComPinConfig(alt, lr) => {
                buffer[0] = 0xDA;
                buffer[1] = 0x2 | ((alt as u8) << 4) | ((lr as u8) << 5);

                &buffer[..2]
            }
            Command::DisplayClockDiv(fosc, div) => {
                buffer[0] = 0xD5;
                buffer[1] = ((fosc & 0xF) << 4) | (div & 0xF);

                &buffer[..2]
            }
            Command::PreChargePeriod(phase1, phase2) => {
                buffer[0] = 0xD9;
                buffer[1] = ((phase2 & 0xF) << 4) | (phase1 & 0xF);

                &buffer[..2]
            }
            Command::VcomhDeselect(level) => {
                buffer[0] = 0xDB;
                buffer[1] = level as u8;

                &buffer[..2]
            }
            Command::ChargePump(en) => {
                buffer[0] = 0x8D;
                buffer[1] = 0x10 | ((en as u8) << 2);

                &buffer[..2]
            }
            Command::InternalIref(en, current) => {
                buffer[0] = 0xAD;
                buffer[1] = ((current as u8) << 5) | ((en as u8) << 4);

                &buffer[..2]
            }
            Command::Noop => {
                buffer[0] = 0xE3;

                &buffer[..1]
            }
            Command::FastNoop => &[],
        }
    }

    /// Send command to SSD1306
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        let mut buffer = [0; 7];
        let data = self.encode(&mut buffer);

        if !data.is_empty() {
            // Send command over the interface
            iface.send_commands(U8(data))
        } else {
            Ok(())
        }
    }

    /// Send command to SSD1306 asynchronously
    pub async fn send_async<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: display_interface::AsyncWriteOnlyDataCommand,
    {
        let mut buffer = [0; 7];
        let data = self.encode(&mut buffer);

        if !data.is_empty() {
            // Send command over the interface
            iface.send_commands(U8(data)).await
        } else {
            Ok(())
        }
    }
}

/// Horizontal Scroll Direction
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum HScrollDir {
    /// Left to right
    LeftToRight = 0,
    /// Right to left
    RightToLeft = 1,
}

/// Vertical and horizontal scroll dir
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum VHScrollDir {
    /// Vertical and right horizontal
    VerticalRight = 0b01,
    /// Vertical and left horizontal
    VerticalLeft = 0b10,
}

/// Display page
#[derive(Debug, Clone, Copy)]
pub enum Page {
    /// Page 0
    Page0 = 0b0000,
    /// Page 1
    Page1 = 0b0001,
    /// Page 2
    Page2 = 0b0010,
    /// Page 3
    Page3 = 0b0011,
    /// Page 4
    Page4 = 0b0100,
    /// Page 5
    Page5 = 0b0101,
    /// Page 6
    Page6 = 0b0110,
    /// Page 7
    Page7 = 0b0111,
    /// Page 8
    Page8 = 0b1000,
    /// Page 9
    Page9 = 0b1001,
    /// Page 10
    Page10 = 0b1010,
    /// Page 11
    Page11 = 0b1011,
    /// Page 12
    Page12 = 0b1100,
    /// Page 13
    Page13 = 0b1101,
    /// Page 14
    Page14 = 0b1110,
    /// Page 15
    Page15 = 0b1111,
}

impl From<u8> for Page {
    fn from(val: u8) -> Page {
        match val / 8 {
            0 => Page::Page0,
            1 => Page::Page1,
            2 => Page::Page2,
            3 => Page::Page3,
            4 => Page::Page4,
            5 => Page::Page5,
            6 => Page::Page6,
            7 => Page::Page7,
            8 => Page::Page8,
            9 => Page::Page9,
            10 => Page::Page10,
            11 => Page::Page11,
            12 => Page::Page12,
            13 => Page::Page13,
            14 => Page::Page14,
            15 => Page::Page15,
            _ => panic!("Page too high"),
        }
    }
}

/// Frame interval
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum NFrames {
    /// 2 Frames
    F2 = 0b111,
    /// 3 Frames
    F3 = 0b100,
    /// 4 Frames
    F4 = 0b101,
    /// 5 Frames
    F5 = 0b000,
    /// 25 Frames
    F25 = 0b110,
    /// 64 Frames
    F64 = 0b001,
    /// 128 Frames
    F128 = 0b010,
    /// 256 Frames
    F256 = 0b011,
}

/// Address mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AddrMode {
    /// Horizontal mode
    Horizontal = 0b00,
    /// Vertical mode
    Vertical = 0b01,
    /// Page mode (default)
    Page = 0b10,
}

/// Vcomh Deselect level
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum VcomhLevel {
    /// 0.65 * Vcc
    V065 = 0b001,
    /// 0.77 * Vcc
    V077 = 0b010,
    /// 0.83 * Vcc
    V083 = 0b011,
    /// Auto
    Auto = 0b100,
}
