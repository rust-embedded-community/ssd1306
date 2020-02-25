//! Unbuffered terminal display mode
//!
//! This mode uses the 7x7 pixel [MarioChrome](https://github.com/techninja/MarioChron/) font to
//! draw characters to the display without needing a framebuffer. It will write characters from top
//! left to bottom right in an 8x8 pixel grid, restarting at the top left of the display once full.
//! The display itself takes care of wrapping lines.
//!
//! ```rust
//! # use ssd1306::test_helpers::I2cStub;
//! # let i2c = I2cStub;
//! use core::fmt::Write;
//! use ssd1306::{mode::TerminalMode, Builder};
//!
//! let mut display: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//! display.clear().unwrap();
//!
//! // Print a-zA-Z
//! for c in 97..123 {
//!     display
//!         .write_str(unsafe { core::str::from_utf8_unchecked(&[c]) })
//!         .unwrap();
//! }
//! ```

use crate::{
    command::AddrMode,
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    interface::DisplayInterface,
    mode::{
        displaymode::DisplayModeTrait,
        terminal::TerminalModeError::{InterfaceError, OutOfBounds, Uninitialized},
    },
    properties::DisplayProperties,
    Error,
};
use core::{cmp::min, fmt};
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// Contains the new row that the cursor has wrapped around to
struct CursorWrapEvent(u8);

struct Cursor {
    col: u8,
    row: u8,
    width: u8,
    height: u8,
}

impl Cursor {
    pub fn new(width_pixels: u8, height_pixels: u8) -> Self {
        let width = width_pixels / 8;
        let height = height_pixels / 8;
        Cursor {
            col: 0,
            row: 0,
            width,
            height,
        }
    }

    /// Advances the logical cursor by one character.
    /// Returns a value indicating if this caused the cursor to wrap to the next line or the next screen.
    pub fn advance(&mut self) -> Option<CursorWrapEvent> {
        self.col = (self.col + 1) % self.width;
        if self.col == 0 {
            self.row = (self.row + 1) % self.height;
            Some(CursorWrapEvent(self.row))
        } else {
            None
        }
    }

    /// Advances the logical cursor to the start of the next line
    /// Returns a value indicating the now active line
    pub fn advance_line(&mut self) -> CursorWrapEvent {
        self.row = (self.row + 1) % self.height;
        self.col = 0;
        CursorWrapEvent(self.row)
    }

    /// Sets the position of the logical cursor arbitrarily.
    /// The position will be capped at the maximal possible position.
    pub fn set_position(&mut self, col: u8, row: u8) {
        self.col = min(col, self.width - 1);
        self.row = min(row, self.height - 1);
    }

    /// Gets the position of the logical cursor on screen in (col, row) order
    pub fn get_position(&self) -> (u8, u8) {
        (self.col, self.row)
    }

    /// Gets the logical dimensions of the screen in terms of characters, as (width, height)
    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.width, self.height)
    }
}

/// Errors which can occur when interacting with the terminal mode
#[derive(Clone)]
pub enum TerminalModeError<DI>
where
    DI: DisplayInterface,
{
    /// An error occurred in the underlying interface layer
    InterfaceError(DI::Error),
    /// The mode was used before it was initialized
    Uninitialized,
    /// A location was specified outside the bounds of the screen
    OutOfBounds,
}

impl<DI> core::fmt::Debug for TerminalModeError<DI>
where
    DI: DisplayInterface,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            InterfaceError(_) => "InterfaceError".fmt(f),
            Uninitialized => "Uninitialized".fmt(f),
            OutOfBounds => "OutOfBound".fmt(f),
        }
    }
}

// Cannot use From<_> due to coherence
trait IntoTerminalModeResult<DI, T>
where
    DI: DisplayInterface,
{
    fn terminal_err(self) -> Result<T, TerminalModeError<DI>>;
}

impl<DI, T> IntoTerminalModeResult<DI, T> for Result<T, DI::Error>
where
    DI: DisplayInterface,
{
    fn terminal_err(self) -> Result<T, TerminalModeError<DI>> {
        self.map_err(|err| InterfaceError(err))
    }
}

// TODO: Add to prelude
/// Terminal mode handler
pub struct TerminalMode<DI> {
    properties: DisplayProperties<DI>,
    cursor: Option<Cursor>,
}

impl<DI> DisplayModeTrait<DI> for TerminalMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new TerminalMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        TerminalMode {
            properties,
            cursor: None,
        }
    }

    /// Release all resources used by TerminalMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> TerminalMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display and reset the cursor to the top left corner
    pub fn clear(&mut self) -> Result<(), TerminalModeError<DI>> {
        let display_size = self.properties.get_size();

        // The number of characters that can fit on the display at once (w * h / 8 * 8)
        // TODO: Use `display_size.dimensions()`
        let numchars = match display_size {
            DisplaySize::Display128x64 => 128,
            DisplaySize::Display128x32 => 64,
            DisplaySize::Display96x16 => 24,
            DisplaySize::Display72x40 => 45,
            DisplaySize::Display64x48 => 48,
        };

        // Let the chip handle line wrapping so we can fill the screen with blanks faster
        self.properties
            .change_mode(AddrMode::Horizontal)
            .terminal_err()?;
        let (display_width, display_height) = self.properties.get_dimensions();
        let (display_x_offset, display_y_offset) = self.properties.display_offset;
        self.properties
            .set_draw_area(
                (display_x_offset, display_y_offset),
                (
                    display_width + display_x_offset,
                    display_height + display_y_offset,
                ),
            )
            .terminal_err()?;

        // Clear the display
        for _ in 0..numchars {
            self.properties.draw(&[0; 8]).terminal_err()?;
        }

        // But for normal operation we manage the line wrapping
        self.properties.change_mode(AddrMode::Page).terminal_err()?;
        self.reset_pos()?;

        Ok(())
    }

    /// Reset display
    pub fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(10);
        rst.set_high().map_err(Error::Pin)
    }

    /// Write out data to display. This is a noop in terminal mode.
    pub fn flush(&mut self) -> Result<(), TerminalModeError<DI>> {
        Ok(())
    }

    /// Print a character to the display
    pub fn print_char(&mut self, c: char) -> Result<(), TerminalModeError<DI>> {
        match c {
            '\n' => {
                let CursorWrapEvent(new_line) = self.ensure_cursor()?.advance_line();
                self.properties.set_column(0).terminal_err()?;
                self.properties.set_row(new_line * 8).terminal_err()?;
            }
            '\r' => {
                self.properties.set_column(0).terminal_err()?;
                let (_, cur_line) = self.ensure_cursor()?.get_position();
                self.ensure_cursor()?.set_position(0, cur_line);
            }
            _ => {
                // Send the pixel data to the display
                self.properties
                    .draw(&Self::char_to_bitmap(c))
                    .terminal_err()?;
                // Increment character counter and potentially wrap line
                self.advance_cursor()?;
            }
        }

        Ok(())
    }

    /// Initialise the display in page mode (i.e. a byte walks down a column of 8 pixels) with
    /// column 0 on the left and column _(display_width - 1)_ on the right, but no automatic line
    /// wrapping.
    pub fn init(&mut self) -> Result<(), TerminalModeError<DI>> {
        self.properties
            .init_with_mode(AddrMode::Page)
            .terminal_err()?;
        self.reset_pos()?;
        Ok(())
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), TerminalModeError<DI>> {
        // we don't need to touch the cursor because rotating 90º or 270º currently just flips
        self.properties.set_rotation(rot).terminal_err()
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn display_on(&mut self, on: bool) -> Result<(), TerminalModeError<DI>> {
        self.properties.display_on(on).terminal_err()
    }

    /// Get the current cursor position, in character coordinates.
    /// This is the (column, row) that the next character will be written to.
    pub fn get_position(&self) -> Result<(u8, u8), TerminalModeError<DI>> {
        self.cursor
            .as_ref()
            .map(|c| c.get_position())
            .ok_or(Uninitialized)
    }

    /// Set the cursor position, in character coordinates.
    /// This is the (column, row) that the next character will be written to.
    /// If the position is out of bounds, an Err will be returned.
    pub fn set_position(&mut self, column: u8, row: u8) -> Result<(), TerminalModeError<DI>> {
        let (width, height) = self.ensure_cursor()?.get_dimensions();
        if column >= width || row >= height {
            Err(OutOfBounds)
        } else {
            self.properties.set_column(column * 8).terminal_err()?;
            self.properties.set_row(row * 8).terminal_err()?;
            self.ensure_cursor()?.set_position(column, row);
            Ok(())
        }
    }

    /// Reset the draw area and move pointer to the top left corner
    fn reset_pos(&mut self) -> Result<(), TerminalModeError<DI>> {
        let (display_x_offset, display_y_offset) = self.properties.display_offset;
        self.properties
            .set_column(display_x_offset)
            .terminal_err()?;
        self.properties.set_row(display_y_offset).terminal_err()?;
        // Initialise the counter when we know it's valid
        let (display_width, display_height) = self.properties.get_dimensions();
        self.cursor = Some(Cursor::new(display_width, display_height));

        Ok(())
    }

    /// Advance the cursor, automatically wrapping lines and/or screens if necessary
    /// Takes in an already-unwrapped cursor to avoid re-unwrapping
    fn advance_cursor(&mut self) -> Result<(), TerminalModeError<DI>> {
        if let Some(CursorWrapEvent(new_row)) = self.ensure_cursor()?.advance() {
            self.properties.set_row(new_row * 8).terminal_err()?;
        }
        Ok(())
    }

    fn ensure_cursor(&mut self) -> Result<&mut Cursor, TerminalModeError<DI>> {
        self.cursor.as_mut().ok_or(Uninitialized)
    }

    fn char_to_bitmap(input: char) -> [u8; 8] {
        // Populate the array with the data from the character array at the right index
        match input {
            '!' => [0x00, 0x00, 0x5F, 0x00, 0x00, 0x00, 0x00, 0x00],
            '"' => [0x00, 0x07, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00],
            '#' => [0x14, 0x7F, 0x14, 0x7F, 0x14, 0x00, 0x00, 0x00],
            '$' => [0x24, 0x2A, 0x7F, 0x2A, 0x12, 0x00, 0x00, 0x00],
            '%' => [0x23, 0x13, 0x08, 0x64, 0x62, 0x00, 0x00, 0x00],
            '&' => [0x36, 0x49, 0x55, 0x22, 0x50, 0x00, 0x00, 0x00],
            '\'' => [0x00, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
            '(' => [0x00, 0x1C, 0x22, 0x41, 0x00, 0x00, 0x00, 0x00],
            ')' => [0x00, 0x41, 0x22, 0x1C, 0x00, 0x00, 0x00, 0x00],
            '*' => [0x08, 0x2A, 0x1C, 0x2A, 0x08, 0x00, 0x00, 0x00],
            '+' => [0x08, 0x08, 0x3E, 0x08, 0x08, 0x00, 0x00, 0x00],
            ',' => [0x00, 0x50, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00],
            '-' => [0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00],
            '.' => [0x00, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00],
            '/' => [0x20, 0x10, 0x08, 0x04, 0x02, 0x00, 0x00, 0x00],
            '0' => [0x1C, 0x3E, 0x61, 0x41, 0x43, 0x3E, 0x1C, 0x00],
            '1' => [0x40, 0x42, 0x7F, 0x7F, 0x40, 0x40, 0x00, 0x00],
            '2' => [0x62, 0x73, 0x79, 0x59, 0x5D, 0x4F, 0x46, 0x00],
            '3' => [0x20, 0x61, 0x49, 0x4D, 0x4F, 0x7B, 0x31, 0x00],
            '4' => [0x18, 0x1C, 0x16, 0x13, 0x7F, 0x7F, 0x10, 0x00],
            '5' => [0x27, 0x67, 0x45, 0x45, 0x45, 0x7D, 0x38, 0x00],
            '6' => [0x3C, 0x7E, 0x4B, 0x49, 0x49, 0x79, 0x30, 0x00],
            '7' => [0x03, 0x03, 0x71, 0x79, 0x0D, 0x07, 0x03, 0x00],
            '8' => [0x36, 0x7F, 0x49, 0x49, 0x49, 0x7F, 0x36, 0x00],
            '9' => [0x06, 0x4F, 0x49, 0x49, 0x69, 0x3F, 0x1E, 0x00],
            ':' => [0x00, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00],
            ';' => [0x00, 0x56, 0x36, 0x00, 0x00, 0x00, 0x00, 0x00],
            '<' => [0x00, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00],
            '=' => [0x14, 0x14, 0x14, 0x14, 0x14, 0x00, 0x00, 0x00],
            '>' => [0x41, 0x22, 0x14, 0x08, 0x00, 0x00, 0x00, 0x00],
            '?' => [0x02, 0x01, 0x51, 0x09, 0x06, 0x00, 0x00, 0x00],
            '@' => [0x32, 0x49, 0x79, 0x41, 0x3E, 0x00, 0x00, 0x00],
            'A' => [0x7E, 0x11, 0x11, 0x11, 0x7E, 0x00, 0x00, 0x00],
            'B' => [0x7F, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00, 0x00],
            'C' => [0x3E, 0x41, 0x41, 0x41, 0x22, 0x00, 0x00, 0x00],
            'D' => [0x7F, 0x7F, 0x41, 0x41, 0x63, 0x3E, 0x1C, 0x00],
            'E' => [0x7F, 0x49, 0x49, 0x49, 0x41, 0x00, 0x00, 0x00],
            'F' => [0x7F, 0x09, 0x09, 0x01, 0x01, 0x00, 0x00, 0x00],
            'G' => [0x3E, 0x41, 0x41, 0x51, 0x32, 0x00, 0x00, 0x00],
            'H' => [0x7F, 0x08, 0x08, 0x08, 0x7F, 0x00, 0x00, 0x00],
            'I' => [0x00, 0x41, 0x7F, 0x41, 0x00, 0x00, 0x00, 0x00],
            'J' => [0x20, 0x40, 0x41, 0x3F, 0x01, 0x00, 0x00, 0x00],
            'K' => [0x7F, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00],
            'L' => [0x7F, 0x7F, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00],
            'M' => [0x7F, 0x02, 0x04, 0x02, 0x7F, 0x00, 0x00, 0x00],
            'N' => [0x7F, 0x04, 0x08, 0x10, 0x7F, 0x00, 0x00, 0x00],
            'O' => [0x3E, 0x7F, 0x41, 0x41, 0x41, 0x7F, 0x3E, 0x00],
            'P' => [0x7F, 0x09, 0x09, 0x09, 0x06, 0x00, 0x00, 0x00],
            'Q' => [0x3E, 0x41, 0x51, 0x21, 0x5E, 0x00, 0x00, 0x00],
            'R' => [0x7F, 0x7F, 0x11, 0x31, 0x79, 0x6F, 0x4E, 0x00],
            'S' => [0x46, 0x49, 0x49, 0x49, 0x31, 0x00, 0x00, 0x00],
            'T' => [0x01, 0x01, 0x7F, 0x01, 0x01, 0x00, 0x00, 0x00],
            'U' => [0x3F, 0x40, 0x40, 0x40, 0x3F, 0x00, 0x00, 0x00],
            'V' => [0x1F, 0x20, 0x40, 0x20, 0x1F, 0x00, 0x00, 0x00],
            'W' => [0x7F, 0x7F, 0x38, 0x1C, 0x38, 0x7F, 0x7F, 0x00],
            'X' => [0x63, 0x14, 0x08, 0x14, 0x63, 0x00, 0x00, 0x00],
            'Y' => [0x03, 0x04, 0x78, 0x04, 0x03, 0x00, 0x00, 0x00],
            'Z' => [0x61, 0x51, 0x49, 0x45, 0x43, 0x00, 0x00, 0x00],
            '[' => [0x00, 0x00, 0x7F, 0x41, 0x41, 0x00, 0x00, 0x00],
            '\\' => [0x02, 0x04, 0x08, 0x10, 0x20, 0x00, 0x00, 0x00],
            ']' => [0x41, 0x41, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
            '^' => [0x04, 0x02, 0x01, 0x02, 0x04, 0x00, 0x00, 0x00],
            '_' => [0x40, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00, 0x00],
            '`' => [0x00, 0x01, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00],
            'a' => [0x20, 0x54, 0x54, 0x54, 0x78, 0x00, 0x00, 0x00],
            'b' => [0x7F, 0x48, 0x44, 0x44, 0x38, 0x00, 0x00, 0x00],
            'c' => [0x38, 0x44, 0x44, 0x44, 0x20, 0x00, 0x00, 0x00],
            'd' => [0x38, 0x44, 0x44, 0x48, 0x7F, 0x00, 0x00, 0x00],
            'e' => [0x38, 0x54, 0x54, 0x54, 0x18, 0x00, 0x00, 0x00],
            'f' => [0x08, 0x7E, 0x09, 0x01, 0x02, 0x00, 0x00, 0x00],
            'g' => [0x08, 0x14, 0x54, 0x54, 0x3C, 0x00, 0x00, 0x00],
            'h' => [0x7F, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00, 0x00],
            'i' => [0x00, 0x44, 0x7D, 0x40, 0x00, 0x00, 0x00, 0x00],
            'j' => [0x20, 0x40, 0x44, 0x3D, 0x00, 0x00, 0x00, 0x00],
            'k' => [0x00, 0x7F, 0x10, 0x28, 0x44, 0x00, 0x00, 0x00],
            'l' => [0x00, 0x41, 0x7F, 0x40, 0x00, 0x00, 0x00, 0x00],
            'm' => [0x7C, 0x04, 0x18, 0x04, 0x78, 0x00, 0x00, 0x00],
            'n' => [0x7C, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00, 0x00],
            'o' => [0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00, 0x00],
            'p' => [0x7C, 0x14, 0x14, 0x14, 0x08, 0x00, 0x00, 0x00],
            'q' => [0x08, 0x14, 0x14, 0x18, 0x7C, 0x00, 0x00, 0x00],
            'r' => [0x7C, 0x08, 0x04, 0x04, 0x08, 0x00, 0x00, 0x00],
            's' => [0x48, 0x54, 0x54, 0x54, 0x20, 0x00, 0x00, 0x00],
            't' => [0x04, 0x3F, 0x44, 0x40, 0x20, 0x00, 0x00, 0x00],
            'u' => [0x3C, 0x40, 0x40, 0x20, 0x7C, 0x00, 0x00, 0x00],
            'v' => [0x1C, 0x20, 0x40, 0x20, 0x1C, 0x00, 0x00, 0x00],
            'w' => [0x3C, 0x40, 0x30, 0x40, 0x3C, 0x00, 0x00, 0x00],
            'x' => [0x00, 0x44, 0x28, 0x10, 0x28, 0x44, 0x00, 0x00],
            'y' => [0x0C, 0x50, 0x50, 0x50, 0x3C, 0x00, 0x00, 0x00],
            'z' => [0x44, 0x64, 0x54, 0x4C, 0x44, 0x00, 0x00, 0x00],
            '{' => [0x00, 0x08, 0x36, 0x41, 0x00, 0x00, 0x00, 0x00],
            '|' => [0x00, 0x00, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
            '}' => [0x00, 0x41, 0x36, 0x08, 0x00, 0x00, 0x00, 0x00],
            _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        }
    }
}

impl<DI> fmt::Write for TerminalMode<DI>
where
    DI: DisplayInterface,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        s.chars().map(move |c| self.print_char(c)).last();
        Ok(())
    }
}
