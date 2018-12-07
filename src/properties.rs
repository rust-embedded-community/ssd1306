//! Container to store and set display properties

use crate::command::{AddrMode, Command, VcomhLevel};
use crate::displayrotation::DisplayRotation;
use crate::displaysize::DisplaySize;
use crate::interface::DisplayInterface;

/// Display properties struct
pub struct DisplayProperties<DI> {
    iface: DI,
    display_size: DisplaySize,
    display_rotation: DisplayRotation,
}

impl<DI> DisplayProperties<DI>
where
    DI: DisplayInterface,
{
    /// Create new DisplayProperties instance
    pub fn new(
        iface: DI,
        display_size: DisplaySize,
        display_rotation: DisplayRotation,
    ) -> DisplayProperties<DI> {
        DisplayProperties {
            iface,
            display_size,
            display_rotation,
        }
    }

    /// Initialise the display in column mode (i.e. a byte walks down a column of 8 pixels) with
    /// column 0 on the left and column _(display_width - 1)_ on the right.
    pub fn init_column_mode(&mut self) -> Result<(), ()> {
        // TODO: Break up into nice bits so display modes can pick whathever they need
        let (_, display_height) = self.display_size.dimensions();

        let display_rotation = self.display_rotation;

        Command::DisplayOn(false).send(&mut self.iface)?;
        Command::DisplayClockDiv(0x8, 0x0).send(&mut self.iface)?;
        Command::Multiplex(display_height - 1).send(&mut self.iface)?;
        Command::DisplayOffset(0).send(&mut self.iface)?;
        Command::StartLine(0).send(&mut self.iface)?;
        // TODO: Ability to turn charge pump on/off
        Command::ChargePump(true).send(&mut self.iface)?;
        Command::AddressMode(AddrMode::Horizontal).send(&mut self.iface)?;

        self.set_rotation(display_rotation)?;

        match self.display_size {
            DisplaySize::Display128x32 => Command::ComPinConfig(false, false).send(&mut self.iface),
            DisplaySize::Display128x64 => Command::ComPinConfig(true, false).send(&mut self.iface),
            DisplaySize::Display96x16 => Command::ComPinConfig(false, false).send(&mut self.iface),
        }?;

        Command::Contrast(0x8F).send(&mut self.iface)?;
        Command::PreChargePeriod(0x1, 0xF).send(&mut self.iface)?;
        Command::VcomhDeselect(VcomhLevel::Auto).send(&mut self.iface)?;
        Command::AllOn(false).send(&mut self.iface)?;
        Command::Invert(false).send(&mut self.iface)?;
        Command::EnableScroll(false).send(&mut self.iface)?;
        Command::DisplayOn(true).send(&mut self.iface)?;

        Ok(())
    }

    /// Set the position in the framebuffer of the display where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), ()> {
        Command::ColumnAddress(start.0, end.0 - 1).send(&mut self.iface)?;
        Command::PageAddress(start.1.into(), (end.1 - 1).into()).send(&mut self.iface)?;
        Ok(())
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), ()> {
        self.iface.send_data(&buffer)?;
        Ok(())
    }

    /// Get the configured display size
    pub fn get_size(&self) -> DisplaySize {
        self.display_size
    }

    // TODO: Replace (u8, u8) with a dimensioney type for consistency
    // TOOD: Make doc tests work
    /// Get display dimensions, taking into account the current rotation of the display
    ///
    /// ```rust
    /// # struct FakeInterface;
    /// #
    /// # impl DisplayInterface for FakeInterface {
    /// #     fn send_command(&mut self, cmd: u8) -> Result<(), ()> { Ok(()) }
    /// #     fn send_data(&mut self, buf: &[u8]) -> Result<(), ()> { Ok(()) }
    /// # }
    /// #
    /// # let interface = FakeInterface {};
    /// #
    /// let disp = DisplayProperties::new(
    ///     interface,
    ///     DisplaySize::Display128x64,
    ///     DisplayRotation::Rotate0
    /// );
    /// assert_eq!(disp.get_dimensions(), (128, 64));
    ///
    /// # let interface = FakeInterface {};
    /// let rotated_disp = DisplayProperties::new(
    ///     interface,
    ///     DisplaySize::Display128x64,
    ///     DisplayRotation::Rotate90
    /// );
    /// assert_eq!(rotated_disp.get_dimensions(), (64, 128));
    /// ```
    pub fn get_dimensions(&self) -> (u8, u8) {
        let (w, h) = self.display_size.dimensions();

        match self.display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (w, h),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (h, w),
        }
    }

    /// Get the display rotation
    pub fn get_rotation(&self) -> DisplayRotation {
        self.display_rotation
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, display_rotation: DisplayRotation) -> Result<(), ()> {
        self.display_rotation = display_rotation;

        match display_rotation {
            DisplayRotation::Rotate0 => {
                Command::SegmentRemap(true).send(&mut self.iface)?;
                Command::ReverseComDir(true).send(&mut self.iface)?;
            }
            DisplayRotation::Rotate90 => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(true).send(&mut self.iface)?;
            }
            DisplayRotation::Rotate180 => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)?;
            }
            DisplayRotation::Rotate270 => {
                Command::SegmentRemap(true).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)?;
            }
        };

        Ok(())
    }
}
