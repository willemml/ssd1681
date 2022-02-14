//! Graphics Support for EPDs
/// TODO: Tests (maybe)

use crate::color::{
    Color,
};
use crate::{HEIGHT, WIDTH};
use embedded_graphics_core::{
    geometry::{
        Dimensions,
        Size,
        Point,
    },
    pixelcolor::GrayColor,
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
    /// Returns buffer 1
    fn buffer1(&self) -> &[u8];
    /// Returns buffer 2
    fn buffer2(&self) -> &[u8];
    /// Returns both buffers
    fn buffers(&self) -> (&[u8],&[u8]);
    /// Sets the rotation of the display
    fn set_rotation(&mut self, rotation: DisplayRotation);
    /// Get the current rotation of the display
    fn rotation(&self) -> DisplayRotation;
    /// Inverts the display for B/W mode. The gray4 mode has black and white swapped (for now).
    fn invert_display(&mut self);
}

/// Display for a 200x200 panel
pub struct Display1in54 {
    buffer:([u8;(WIDTH*HEIGHT)/8],[u8;(WIDTH*HEIGHT)/8]),
    rotation: DisplayRotation,
    inverted:bool,
}
impl Display1in54 {
    /// Create a display buffer
    pub fn new()->Self {
        Display1in54 {
            buffer:([0xff;(WIDTH*HEIGHT)/8],[0xff;(WIDTH*HEIGHT)/8]),
            rotation:DisplayRotation::default(),
            inverted:false,
        }
    }
    fn get_color_bits(&self,color:Color)->(bool,bool) {
        let luma=color.luma();
        let color1=(luma&1)==1;
        let color2=((luma>>1)&1)==1;
        if self.inverted {
            return (!color1,!color2);
        } else {
            return (color1,color2);
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
                let color=self.get_color_bits(pixel.1);
                match self.rotation {
                    Rotate0=>{
                        let mut idx=x+(y*WIDTH);
                        let bit=0b10000000>>(idx%8);
                        idx>>=3;
                        if color.0 {
                            self.buffer.0[idx]&=!bit;
                        } else {
                            self.buffer.0[idx]|=bit;
                        }
                        if color.1 {
                            self.buffer.1[idx]&=!bit;
                        } else {
                            self.buffer.1[idx]|=bit;
                        }
                    },
                    Rotate180=>{
                        let mut idx=((WIDTH-1)-x)+(((HEIGHT-1)-y)*WIDTH);
                        let bit=0b10000000>>(idx%8);
                        idx>>=3;
                        if color.0 {
                            self.buffer.0[idx]&=!bit;
                        } else {
                            self.buffer.0[idx]|=bit;
                        }
                        if color.1 {
                            self.buffer.1[idx]&=!bit;
                        } else {
                            self.buffer.1[idx]|=bit;
                        }
                    },
                    _=>todo!(),
                }
            }
        }
        return Ok(());
    }
    fn clear(&mut self,color:Color)->Result<(),Self::Error> {
        self.clear_buffer(color);
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
        let color=self.get_color_bits(color);
        if color.0 {
            self.buffer.0.fill(0xff);
        } else {
            self.buffer.0.fill(0);
        }
        if color.1 {
            self.buffer.1.fill(0xff);
        } else {
            self.buffer.1.fill(0);
        }
    }
    fn invert_display(&mut self) {
        self.inverted=!self.inverted;
        for (c1,c2) in self.buffer.0.iter_mut().zip(self.buffer.1.iter_mut()) {
            *c1=!*c1;
            *c2=!*c2;
        }
    }
    fn buffer1(&self)->&[u8] {
        &self.buffer.0
    }
    fn buffer2(&self)->&[u8] {
        &self.buffer.1
    }
    fn buffers(&self)->(&[u8],&[u8]) {
        (&self.buffer.0,&self.buffer.1)
    }
    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }
    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}
