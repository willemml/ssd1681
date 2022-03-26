//! Display interface using SPI

use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_hal::{
    delay::blocking::DelayUs,
    spi::blocking::{Write, Transactional, Operation},
    digital::blocking::{InputPin, OutputPin},
};

const RESET_DELAY_MS: u32 = 10;

/// The Connection Interface of all (?) Waveshare EPD-Devices
///
pub(crate) struct DisplayInterface<SPI, BUSY, RST> {
    /// SPI
    _spi: PhantomData<SPI>,
    /// Low for busy, Wait until display is ready!
    busy: BUSY,
    /// Pin for Reseting
    rst: RST,
}

impl<SPI, BUSY, RST> DisplayInterface<SPI, BUSY, RST>
where
    SPI: Write + Transactional,
    BUSY: InputPin,
    RST: OutputPin,
    RST::Error: Debug,
{
    /// Create and initialize display
    pub fn new(busy: BUSY, rst: RST) -> Self {
        DisplayInterface {
            _spi: PhantomData,
            busy,
            rst,
        }
    }

    /// Basic function for sending commands
    pub(crate) fn cmd(&mut self, spi: &mut SPI, command: u8) -> Result<(), SPI::Error> {
        spi.exec(&mut [Operation::Transfer(&mut [0], &[command])])
    }

    /// Basic function for sending an array of u8-values of data over spi
    pub(crate) fn data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        spi.write(data)
    }

    /// Basic function for sending a command and the data belonging to it.
    pub(crate) fn cmd_with_data(
        &mut self,
        spi: &mut SPI,
        command: u8,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.cmd(spi, command)?;
        self.data(spi, data)
    }

    /// Basic function for sending the same byte of data (one u8) multiple times over spi
    /// Used for setting one color for the whole frame
    pub(crate) fn data_x_times(
        &mut self,
        spi: &mut SPI,
        val: u8,
        repetitions: u32,
    ) -> Result<(), SPI::Error> {
        // Transfer data (u8) over spi
        for _ in 0..repetitions {
            self.data(spi, &[val])?;
        }
        Ok(())
    }

    /// Waits until device isn't busy anymore (busy == HIGH)
    pub(crate) fn wait_until_idle(&mut self) {
        while self.busy.is_high().unwrap_or(true) {}
    }

    /// Resets the device.
    pub(crate) fn reset<DELAY: DelayUs>(&mut self, delay: &mut DELAY)->Result<(),DELAY::Error> {
        self.rst.set_low().unwrap();
        delay.delay_ms(RESET_DELAY_MS)?;
        self.rst.set_high().unwrap();
        delay.delay_ms(RESET_DELAY_MS)
    }
}
