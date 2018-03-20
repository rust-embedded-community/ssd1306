// Shamefully taken from https://github.com/EdgewaterDevelopment/rust-ssd1306

use super::interface::DisplayInterface;

/// SSD1306 Commands

/// Commands
#[derive(Debug)]
#[allow(dead_code)]
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
    /// Set multipex ratio from 15-63 (MUX-1)
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
    /// Set up phase 1 and 2 of precharge period. each value is from 0-63
    PreChargePeriod(u8, u8),
    /// Set Vcomh Deselect level
    VcomhDeselect(VcomhLevel),
    /// NOOP
    Noop,
    /// Enable charge pump
    ChargePump(bool),
}

impl Command {
    /// Send command to SSD1306
    pub fn send<DI>(&self, iface: &mut DI) -> Result<(), DI::Error>
    where
        DI: DisplayInterface,
    {
        match *self {
            Command::Contrast(val) => {
                iface.send_command(0x81)?;
                iface.send_command(val)?;
            }
            Command::AllOn(on) => {
                iface.send_command(0xA4 | (on as u8))?;
            }
            Command::Invert(inv) => {
                iface.send_command(0xA6 | (inv as u8))?;
            }
            Command::DisplayOn(on) => {
                iface.send_command(0xAE | (on as u8))?;
            }
            Command::HScrollSetup(dir, start, end, rate) => {
                iface.send_command(0x26 | (dir as u8))?;
                iface.send_command(0)?;
                iface.send_command(start as u8)?;
                iface.send_command(rate as u8)?;
                iface.send_command(end as u8)?;
                iface.send_command(0)?;
                iface.send_command(0xFF)?;
            }
            Command::VHScrollSetup(dir, start, end, rate, offset) => {
                iface.send_command(0x28 | (dir as u8))?;
                iface.send_command(0)?;
                iface.send_command(start as u8)?;
                iface.send_command(rate as u8)?;
                iface.send_command(end as u8)?;
                iface.send_command(offset)?;
            }
            Command::EnableScroll(en) => {
                iface.send_command(0x2E | (en as u8))?;
            }
            Command::VScrollArea(above, lines) => {
                iface.send_command(0xA3)?;
                iface.send_command(above)?;
                iface.send_command(lines)?;
            }
            Command::LowerColStart(addr) => {
                iface.send_command(0xF & addr)?;
            }
            Command::UpperColStart(addr) => {
                iface.send_command(0x1F & addr)?;
            }
            Command::AddressMode(mode) => {
                iface.send_command(0x20)?;
                iface.send_command(mode as u8)?;
            }
            Command::ColumnAddress(start, end) => {
                iface.send_command(0x21)?;
                iface.send_command(start)?;
                iface.send_command(end)?;
            }
            Command::PageAddress(start, end) => {
                iface.send_command(0x22)?;
                iface.send_command(start as u8)?;
                iface.send_command(end as u8)?;
            }
            Command::PageStart(page) => {
                iface.send_command(0xB0 | (page as u8))?;
            }
            Command::StartLine(line) => {
                iface.send_command(0x40 | (0x3F & line))?;
            }
            Command::SegmentRemap(remap) => {
                iface.send_command(0xA0 | (remap as u8))?;
            }
            Command::Multiplex(ratio) => {
                iface.send_command(0xA8)?;
                iface.send_command(ratio)?;
            }
            Command::ReverseComDir(rev) => {
                iface.send_command(0xC0 | ((rev as u8) << 3))?;
            }
            Command::DisplayOffset(offset) => {
                iface.send_command(0xD3)?;
                iface.send_command(offset)?;
            }
            Command::ComPinConfig(alt, lr) => {
                iface.send_command(0xDA)?;
                iface.send_command(0x2 | ((alt as u8) << 4) | ((lr as u8) << 5))?;
            }
            Command::DisplayClockDiv(fosc, div) => {
                iface.send_command(0xD5)?;
                iface.send_command(((0xF & fosc) << 4) | (0xF & div))?;
            }
            Command::PreChargePeriod(phase1, phase2) => {
                iface.send_command(0xD9)?;
                iface.send_command(((0xF & phase2) << 4) | (0xF & phase1))?;
            }
            Command::VcomhDeselect(level) => {
                iface.send_command(0xDB)?;
                iface.send_command((level as u8) << 4)?;
            }
            Command::Noop => {
                iface.send_command(0xE3)?;
            }
            Command::ChargePump(en) => {
                iface.send_command(0x8D)?;
                iface.send_command(0x10 | ((en as u8) << 2))?;
            }
        }

        Ok(())
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
    Page0 = 0b000,
    /// Page 1
    Page1 = 0b001,
    /// Page 2
    Page2 = 0b010,
    /// Page 3
    Page3 = 0b011,
    /// Page 4
    Page4 = 0b100,
    /// Page 5
    Page5 = 0b101,
    /// Page 6
    Page6 = 0b110,
    /// Page 7
    Page7 = 0b111,
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
#[derive(Debug, Clone, Copy)]
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
