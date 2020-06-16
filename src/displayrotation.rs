//! Display rotation

// These are not instantiated, so no need to implement Copy
#![allow(missing_copy_implementations)]

use crate::command::Command;
use display_interface::{DisplayError, WriteOnlyDataCommand};

/// A valid display rotation value
pub trait DisplayRotationType {
    /// Send rotation related configuration
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError>;

    /// Flip coordinates if necessary
    fn transform(&self, x: u8, y: u8) -> (u8, u8);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Display rotation values
pub enum DisplayRotation {
    /// No image rotation
    Rotate0,
    /// Rotate the image 90º clockwise
    Rotate90,
    /// Rotate the image 180º clockwise
    Rotate180,
    /// Rotate the image 270º clockwise
    Rotate270,
}

impl DisplayRotation {
    /// ...
    pub fn is_0(self) -> bool {
        self == DisplayRotation::Rotate0
    }

    /// ...
    pub fn is_90(self) -> bool {
        self == DisplayRotation::Rotate90
    }

    /// ...
    pub fn is_180(self) -> bool {
        self == DisplayRotation::Rotate180
    }

    /// ...
    pub fn is_270(self) -> bool {
        self == DisplayRotation::Rotate270
    }
}

/// Using this rotation type enables changing the display rotation in runtime.
pub struct DynamicRotation {
    rotation: DisplayRotation,
}

impl DynamicRotation {
    /// Create a DynamicRontation object from the given rotation value
    pub fn with_rotation(rotation: DisplayRotation) -> Self {
        Self { rotation }
    }

    /// Change display rotation
    pub fn set_rotation(
        &mut self,
        rotation: DisplayRotation,
        iface: &mut impl WriteOnlyDataCommand,
    ) -> Result<(), DisplayError> {
        if core::mem::replace(&mut self.rotation, rotation) != rotation {
            self.configure(iface)
        } else {
            Ok(())
        }
    }

    /// Return the current rotation
    pub fn get_rotation(&self) -> DisplayRotation {
        self.rotation
    }
}

impl Default for DynamicRotation {
    fn default() -> Self {
        Self::with_rotation(DisplayRotation::Rotate0)
    }
}

/// Implementors of this trait support changing display rotation in runtime.
pub trait Rotatable {
    /// Change display rotation
    fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), DisplayError>;

    /// Return the current rotation
    fn get_rotation(&self) -> DisplayRotation;
}

impl DisplayRotationType for DynamicRotation {
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        match self.rotation {
            DisplayRotation::Rotate0 => {
                Command::SegmentRemap(true).send(iface)?;
                Command::ReverseComDir(true).send(iface)?;
            }
            DisplayRotation::Rotate90 => {
                Command::SegmentRemap(false).send(iface)?;
                Command::ReverseComDir(true).send(iface)?;
            }
            DisplayRotation::Rotate180 => {
                Command::SegmentRemap(false).send(iface)?;
                Command::ReverseComDir(false).send(iface)?;
            }
            DisplayRotation::Rotate270 => {
                Command::SegmentRemap(true).send(iface)?;
                Command::ReverseComDir(false).send(iface)?;
            }
        }
        Ok(())
    }

    fn transform(&self, x: u8, y: u8) -> (u8, u8) {
        match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (x, y),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (y, x),
        }
    }
}

/// No rotation
pub struct Rotate0;
impl DisplayRotationType for Rotate0 {
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::SegmentRemap(true).send(iface)?;
        Command::ReverseComDir(true).send(iface)?;
        Ok(())
    }

    fn transform(&self, x: u8, y: u8) -> (u8, u8) {
        (x, y)
    }
}

/// 90º CW rotation
pub struct Rotate90;
impl DisplayRotationType for Rotate90 {
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::SegmentRemap(false).send(iface)?;
        Command::ReverseComDir(true).send(iface)?;
        Ok(())
    }

    fn transform(&self, x: u8, y: u8) -> (u8, u8) {
        (y, x)
    }
}

/// 180º rotation
pub struct Rotate180;
impl DisplayRotationType for Rotate180 {
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::SegmentRemap(false).send(iface)?;
        Command::ReverseComDir(false).send(iface)?;
        Ok(())
    }

    fn transform(&self, x: u8, y: u8) -> (u8, u8) {
        (x, y)
    }
}

/// 270º CW rotation
pub struct Rotate270;
impl DisplayRotationType for Rotate270 {
    fn configure(&self, iface: &mut impl WriteOnlyDataCommand) -> Result<(), DisplayError> {
        Command::SegmentRemap(true).send(iface)?;
        Command::ReverseComDir(false).send(iface)?;
        Ok(())
    }

    fn transform(&self, x: u8, y: u8) -> (u8, u8) {
        (y, x)
    }
}
