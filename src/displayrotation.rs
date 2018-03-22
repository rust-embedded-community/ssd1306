//! Display rotation

/// Display rotation enumeration
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation, normal display
    Rotate0,
    /// Rotate by 90 degress clockwise
    Rotate90,
    /// Rotate by 180 degress (flip)
    Rotate180,
    /// Rotate the display by 270 degress
    Rotate270,
}
