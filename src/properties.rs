//! Module to handle common display properties
use command::Command;

use displayrotation::DisplayRotation;
use displaysize::DisplaySize;

use interface::DisplayInterface;

/// DisplayProperties
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

    /// Borrow configured interface for raw communication
    pub fn borrow_iface_mut(&mut self) -> &mut DI {
        &mut self.iface
    }

    /// Get the configured display size
    pub fn get_size(&self) -> &DisplaySize {
        &self.display_size
    }

    /// Get the display rotation
    pub fn get_rotation(&self) -> &DisplayRotation {
        &self.display_rotation
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, display_rotation: DisplayRotation) -> Result<(), DI::Error> {
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
