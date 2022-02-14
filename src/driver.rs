//! Driver for interacting with SSD1681 display driver


use core::fmt::Debug;
use embedded_hal::{
    spi::blocking::Write,
    delay::blocking::DelayUs,
    digital::blocking::{InputPin, OutputPin},
};
use embedded_graphics_core::primitives::Rectangle;
use crate::{
    interface::DisplayInterface,
    graphics::Display,
    cmd,
    flag,
    HEIGHT,
    WIDTH,
};


type Waveform=[u8;159];


// The raw data from the C++ file
static FULL_WAVEFORM:Waveform=[
    0x80,0x48,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x40,0x48,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x80,0x48,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x40,0x48,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0xA,0x0,0x0,0x0,0x0,0x0,0x0,
    0x8,0x1,0x0,0x8,0x1,0x0,0x2,
    0xA,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x22,0x22,0x22,0x22,0x22,0x22,0x0,0x0,0x0,
    0x22,0x17,0x41,0x0,0x32,0x20
];
static PARTIAL_WAVEFORM:Waveform=[
    0x0,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x80,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x40,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0xF,0x0,0x0,0x0,0x0,0x0,0x0,
    0x1,0x1,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x22,0x22,0x22,0x22,0x22,0x22,0x0,0x0,0x0,
    0x02,0x17,0x41,0xB0,0x32,0x28,
];
// Taken from a good-display example using the SSD1681 and a B/W display
// https://www.good-display.com/product/388.html
// S-GDEY0154D67-210414.rar/S-GDEY0154D67-P-210414.rar/Display_EPD_W21.c
static GRAY4_WAVEFORM:Waveform=[
    0x40,0x48,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x8,0x48,0x10,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x2,0x48,0x4,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x20,0x48,0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0xA,0x19,0x0,0x3,0x8,0x0,0x0,
    0x14,0x1,0x0,0x14,0x1,0x0,0x3,
    0xA,0x3,0x0,0x8,0x19,0x0,0x0,
    0x1,0x0,0x0,0x0,0x0,0x0,0x1,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x0,0x0,0x0,0x0,0x0,0x0,0x0,
    0x22,0x22,0x22,0x22,0x22,0x22,0x0,0x0,0x0,
    0x22,0x17,0x41,0x0,0x32,0x1C,
];


/// The refresh type. Full or partial.
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum LutType {
    /// Full refresh
    Full,
    /// Partial refresh
    /// -------------------
    /// **WARNING!** May permanently damage the display!
    Partial,
    /// Gray 4 waveform
    /// From testing, it appears that you have to set the LUT to something other that Gray4, then
    /// update the 2 framebuffers, and finally reset the LUT to Gray4 to update the screen.
    /// NOTE: Gray4 has black/white bits inverted (for now), so you will need to update the display
    /// accordingly
    /// -------------------
    /// **WARNING! EXPERIMENTAL AND MAY BREAK YOUR DISPLAY**
    Gray4
}


/// A configured display with a hardware interface.
pub struct Ssd1681<SPI, CS, BUSY, DC, RST> {
    interface: DisplayInterface<SPI, CS, BUSY, DC, RST>,
    window:Option<Rectangle>,
    lut_type:LutType,
}
impl<SPI, CS, BUSY, DC, RST> Ssd1681<SPI, CS, BUSY, DC, RST>
where
    SPI: Write<u8>,
    CS: OutputPin,
    CS::Error: Debug,
    BUSY: InputPin,
    DC: OutputPin,
    DC::Error: Debug,
    RST: OutputPin,
    RST::Error: Debug,
{
    /// Create and initialize the display driver
    pub fn new<DELAY: DelayUs>(
        spi: &mut SPI,
        cs: CS,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, SPI::Error>{
        let interface = DisplayInterface::new(cs, busy, dc, rst);
        let mut ssd1681 = Ssd1681 {interface,window:None,lut_type:LutType::Full};
        ssd1681.init(spi,delay)?;
        Ok(ssd1681)
    }

    /// Initialise the controller
    fn init<DELAY: DelayUs>(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.interface.reset(delay).unwrap();
        self.interface.cmd(spi, cmd::SW_RESET)?;
        self.interface.wait_until_idle();

        self.interface.cmd_with_data(spi, cmd::DRIVER_CONTROL, &[(HEIGHT as u8) - 1, 0x00, 0x00])?;

        self.use_full_frame(spi)?;

        self.interface.cmd_with_data(
            spi,
            cmd::BORDER_WAVEFORM_CONTROL,
            &[flag::BORDER_WAVEFORM_FOLLOW_LUT | flag::BORDER_WAVEFORM_LUT1],
        )?;

        self.interface.cmd_with_data(spi, cmd::DATA_ENTRY_MODE, &[flag::DATA_ENTRY_INCRY_INCRX])?;

        self.interface.cmd_with_data(spi, cmd::TEMP_CONTROL, &[flag::INTERNAL_TEMP_SENSOR])?;

        self.interface.wait_until_idle();
        Ok(())
    }
    /// Sets the current lookup table to `lut_type` and inverts the buffers if needed.
    ///
    /// See [`LutType`] for caveats
    pub fn set_lut<D:Display>(&mut self,spi:&mut SPI,lut_type:LutType,display:&mut D) -> Result<(), SPI::Error> {
        match self.lut_type {   // if we convert TO or FROM `Gray4` LutType, then invert the display
            LutType::Gray4=>{
                if lut_type!=LutType::Gray4 {
                    display.invert_display();
                }
            },
            _=>{
                if lut_type==LutType::Gray4 {
                    display.invert_display();
                }
            },
        }
        let lut=match lut_type {
            LutType::Full=>&FULL_WAVEFORM,
            LutType::Partial=>&PARTIAL_WAVEFORM,
            LutType::Gray4=>&GRAY4_WAVEFORM,
        };
        self.lut_type=lut_type;

        self.interface.cmd_with_data(spi,0x32,&lut[..153])?;
        self.interface.wait_until_idle();

        self.interface.cmd_with_data(spi,0x3f,&[lut[153]])?;
        self.interface.cmd_with_data(spi,0x03,&[lut[154]])?;
        self.interface.cmd_with_data(spi,0x04,&lut[155..=157])?;
        self.interface.cmd_with_data(spi,0x2C,&[lut[158]])?; // VCOM Voltage

        self.interface.cmd(spi,0x2c)?;
        self.interface.wait_until_idle();
        return Ok(());
    }
    /// Returns the currently active lut type
    pub fn current_lut_type(&self)->LutType {self.lut_type}

    /// Update buffer1 on the display driver
    pub fn update_frame1(&mut self, spi: &mut SPI, buffer: &[u8]) -> Result<(), SPI::Error> {
        self.use_full_frame(spi)?;
        self.interface
            .cmd_with_data(spi, cmd::WRITE_BUFFER1_DATA, &buffer)
    }
    /// Update buffer2 on the display driver
    pub fn update_frame2(&mut self, spi: &mut SPI, buffer: &[u8]) -> Result<(), SPI::Error> {
        self.use_full_frame(spi)?;
        self.interface
            .cmd_with_data(spi, cmd::WRITE_BUFFER2_DATA, &buffer)
    }
    /// Takes a buffer implementing [`Display`] and updates the EPD's buffers with it
    pub fn update_frames<D:Display>(&mut self,spi:&mut SPI,buffers:&D)->Result<(),SPI::Error> {
        self.update_frame1(spi,buffers.buffer1())?;
        self.update_frame2(spi,buffers.buffer2())
    }
    /// Sets the current memory window
    pub fn set_window(&mut self,spi:&mut SPI,window:Rectangle)->Result<(),SPI::Error> {
        self.window=Some(window.clone());
        self.use_window(spi,window)
    }
    /// Unsets the current window
    pub fn unset_window(&mut self,spi:&mut SPI)->Result<(),SPI::Error> {
        self.window=None;
        self.use_full_frame(spi)
    }
    /// Gets the current window, if there is one
    pub fn current_window(&self)->Option<Rectangle> {self.window}
    /// Displays part of the buffer
    pub fn display_window(&mut self,spi:&mut SPI)->Result<(),SPI::Error> {
        match self.lut_type {
            LutType::Gray4=>self.interface.cmd_with_data(spi,cmd::UPDATE_DISPLAY_CTRL2,&[flag::GRAY4_DISPLAY_MODE_1])?,
            _=>self.interface.cmd_with_data(spi,cmd::UPDATE_DISPLAY_CTRL2,&[flag::BW_DISPLAY_MODE_1])?,
        }

        self.interface.cmd(spi, cmd::MASTER_ACTIVATE)?;

        self.interface.wait_until_idle();
        return Ok(());
    }
    /// Start an update of the whole display
    pub fn display_frame(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        self.use_full_frame(spi)?;
        match self.lut_type {
            LutType::Gray4=>self.interface.cmd_with_data(spi,cmd::UPDATE_DISPLAY_CTRL2,&[flag::GRAY4_DISPLAY_MODE_1])?,
            _=>self.interface.cmd_with_data(spi,cmd::UPDATE_DISPLAY_CTRL2,&[flag::BW_DISPLAY_MODE_1])?,
        }
        self.interface.cmd(spi,cmd::MASTER_ACTIVATE)?;

        self.interface.wait_until_idle();

        Ok(())
    }

    /// Make the whole black and white frame on the display driver white
    pub fn clear_frame1(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        self.use_full_frame(spi)?;

        let color=match self.lut_type {
            LutType::Gray4=>0xff,
            _=>0,
        };

        self.interface.cmd(spi, cmd::WRITE_BUFFER1_DATA)?;
        self.interface
            .data_x_times(spi, color, (WIDTH as u32) / 8 * (HEIGHT as u32))?;
        Ok(())
    }
    /// Make the whole black and white frame on the display driver white
    pub fn clear_frame2(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        self.use_full_frame(spi)?;

        let color=match self.lut_type {
            LutType::Gray4=>0xff,
            _=>0,
        };

        self.interface.cmd(spi, cmd::WRITE_BUFFER2_DATA)?;
        self.interface
            .data_x_times(spi, color, (WIDTH as u32) / 8 * (HEIGHT as u32))?;
        Ok(())
    }
    /// Make both buffers white
    pub fn clear_frames(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        self.clear_frame1(spi)?;
        self.clear_frame2(spi)
    }

    fn use_full_frame(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        // choose full frame/ram
        self.set_ram_area(spi,0,0,(WIDTH as u32)-1,(HEIGHT as u32)-1)?;

        // start from the beginning
        self.set_ram_counter(spi,0,0)
    }

    fn use_window(&mut self,spi:&mut SPI,window:Rectangle)->Result<(),SPI::Error> {
        let top_left=window.top_left;
        let bottom_right=top_left+window.size;
        self.set_ram_area(spi,top_left.x as u32,top_left.y as u32,bottom_right.x as u32,bottom_right.y as u32)?;
        self.set_ram_counter(spi,top_left.x as u32,top_left.y as u32)
    }

    fn set_ram_area(
        &mut self,
        spi: &mut SPI,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> Result<(), SPI::Error> {
        assert!(start_x < end_x);
        assert!(start_y < end_y);

        self.interface.cmd_with_data(
            spi,
            cmd::SET_RAMXPOS,
            &[(start_x >> 3) as u8,(end_x >> 3) as u8],
        )?;

        self.interface.cmd_with_data(
            spi,
            cmd::SET_RAMYPOS,
            &[
                start_y as u8,
                (start_y >> 8) as u8,
                end_y as u8,
                (end_y >> 8) as u8,
            ],
        )?;
        Ok(())
    }

    fn set_ram_counter(&mut self, spi: &mut SPI, x: u32, y: u32) -> Result<(), SPI::Error> {
        // x is positioned in bytes, so the last 3 bits which show the position inside a byte in the ram
        // aren't relevant
        self.interface
            .cmd_with_data(spi, cmd::SET_RAMX_COUNTER, &[(x >> 3) as u8])?;

        // 2 Databytes: A[7:0] & 0..A[8]
        self.interface
            .cmd_with_data(spi, cmd::SET_RAMY_COUNTER, &[y as u8, (y >> 8) as u8])?;
        Ok(())
    }

    // pub fn wake_up<DELAY: DelayMs<u8>>(
    //     &mut self,
    //     spi: &mut SPI,
    //     delay: &mut DELAY,
    // ) -> Result<(), SPI::Error> {
    //     todo!()
    // }
}
