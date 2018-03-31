//! SSD1306 OLED display driver

#![no_std]
// TODO: Docs
// #![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate embedded_hal as hal;

mod command;
mod displaysize;
pub mod displayrotation;
pub mod builder;
pub mod interface;
pub mod mode;

pub use builder::Builder;
pub use displaysize::DisplaySize;
