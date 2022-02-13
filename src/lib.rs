//! SSD1681 ePaper Display Driver
//!
//! Used in the [Adafruit 1.54" B/W display](https://www.adafruit.com/product/4196)
//!
//! For a complete example see [the example](https://github.com/Clinery1/ssd1681/blob/master/examples/adafruit_1in54_bw.rs).
//!
//! This driver is losely modeled after the
//! [epd-waveshare](https://github.com/caemor/epd-waveshare) drivers but built for my needs.
//!
//!
//! ### Usage
//! To
//! display something you:
//!
//! 1. first create a buffer and draw things onto it, preferably
//! with [`embedded_graphics`](https://github.com/jamwaffles/embedded-graphics).
//! 2. then send the frame to the display driver using [`driver::Ssd1681::update_frame`]
//! 3. then kick off a display update using [`driver::Ssd1681::update_frame`]
//!
//!
#![no_std]
#![deny(missing_docs)]
#![allow(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

pub mod graphics;
pub mod driver;
pub mod interface;

/// Useful exports
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::driver::Ssd1681;

    pub use crate::graphics::{Display, Display1in54, DisplayRotation};
}
mod cmd {
    pub const SW_RESET: u8 = 0x12;
    pub const DRIVER_CONTROL: u8 = 0x01;
    pub const DATA_ENTRY_MODE: u8 = 0x11;
    pub const SET_RAMXPOS: u8 = 0x44;
    pub const SET_RAMYPOS: u8 = 0x45;
    pub const BORDER_WAVEFORM_CONTROL: u8 = 0x3C;
    pub const TEMP_CONTROL: u8 = 0x18;

    // Update
    pub const SET_RAMX_COUNTER: u8 = 0x4E;
    pub const SET_RAMY_COUNTER: u8 = 0x4F;
    pub const WRITE_BW_DATA: u8 = 0x24;
    pub const UPDATE_DISPLAY_CTRL2: u8 = 0x22;
    pub const MASTER_ACTIVATE: u8 = 0x20;
}
mod flag {
    pub const DATA_ENTRY_INCRY_INCRX: u8 = 0b11;
    pub const INTERNAL_TEMP_SENSOR: u8 = 0x80;
    pub const BORDER_WAVEFORM_FOLLOW_LUT: u8 = 0b0100;
    pub const BORDER_WAVEFORM_LUT1: u8 = 0b0001;
    pub const DISPLAY_MODE_1: u8 = 0xF7;
}
/// Reexports of embedded graphics [`BinaryColor`]
pub mod color {
    pub use embedded_graphics_core::pixelcolor::BinaryColor as Color;
    pub use Color::{
        Off as White,
        On as Black,
    };
}

/// Maximum display height this driver supports
pub const HEIGHT:usize=200;
/// Maximum display width this driver supports
pub const WIDTH:usize=200;
