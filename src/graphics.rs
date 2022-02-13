//! Graphics Support for EPDs
/// TODO: Tests (maybe)

use crate::color::{
    Color,
    Black,
};
use crate::{HEIGHT, WIDTH};
use embedded_graphics_core::{
    geometry::{
        Dimensions,
        Size,
        Point,
    },
    draw_target::DrawTarget,
    primitives::Rectangle,
    Pixel,
};

/// Displayrotation
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}
impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}

/// Necessary traits for all displays to implement for drawing
///
/// Adds support for:
/// - Rotations
pub trait Display:DrawTarget {
    /// Sets the entire buffer to the given color
    fn clear_buffer(&mut self,color:Color);
    /// Returns the buffer
    fn buffer(&self) -> &[u8];
    /// Sets the rotation of the display
    fn set_rotation(&mut self, rotation: DisplayRotation);
    /// Get the current rotation of the display
    fn rotation(&self) -> DisplayRotation;
}

/// Display for a 200x200 panel
pub struct Display1in54 {
    buffer:[u8;(WIDTH*HEIGHT)/8],
    rotation: DisplayRotation,
}
impl Display1in54 {
    /// Create a display buffer
    pub fn new()->Self {
        Display1in54 {
            buffer:[0xff;(WIDTH*HEIGHT)/8],
            rotation:DisplayRotation::default(),
        }
    }
}
impl DrawTarget for Display1in54 {
    type Color=Color;
    type Error=core::convert::Infallible;
    fn draw_iter<I:IntoIterator<Item=Pixel<Color>>>(&mut self,pixels:I)->Result<(),Self::Error> {
        use DisplayRotation::*;
        for pixel in pixels {
            let pos=pixel.0;
            if pos.x<(WIDTH as i32)&&pos.y<(HEIGHT as i32)&&pos.x>0&&pos.y>0 {
                let x=pos.x as usize;
                let y=pos.y as usize;
                let color=pixel.1==Black;
                match self.rotation {
                    Rotate0=>{
                        let mut idx=x+(y*WIDTH);
                        let bit=0b10000000>>(idx%8);
                        idx>>=3;
                        if color {
                            self.buffer[idx]&=!bit;
                        } else {
                            self.buffer[idx]|=bit;
                        }
                    },
                    _=>todo!(),
                }
            }
        }
        return Ok(());
    }
}
impl Dimensions for Display1in54 {
    fn bounding_box(&self)->Rectangle {
        Rectangle::new(Point::zero(),Size::new(WIDTH as u32,HEIGHT as u32))
    }
}

impl Display for Display1in54 {
    fn clear_buffer(&mut self,color:Color) {
        if color==Black {
            self.buffer.fill(0);
        } else {
            self.buffer.fill(0xff);
        }
    }
    fn buffer(&self)->&[u8] {
        &self.buffer
    }
    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }
    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}
