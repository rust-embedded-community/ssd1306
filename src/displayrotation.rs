//! Display rotation

// TODO: Add to prelude
/// Display rotation.
///
/// Note that 90º and 270º rotations are not supported by
// [`TerminalMode`](../mode/terminal/struct.TerminalMode.html).
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation, normal display
    Rotate0,
    /// Rotate by 90 degress clockwise
    Rotate90,
    /// Rotate by 180 degress clockwise
    Rotate180,
    /// Rotate 270 degress clockwise
    Rotate270,
}
