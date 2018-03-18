// TODO: Prelude
#[derive(Clone, Copy)]
pub enum DisplaySize {
    Display128x64,
    Display128x32,
}

impl DisplaySize {
    // TODO: Use whatever vec2 impl I decide to use here
    pub fn dimensions(&self) -> (u8, u8) {
        match *self {
            DisplaySize::Display128x64 => (128, 64),
            DisplaySize::Display128x32 => (128, 32),
        }
    }
}
