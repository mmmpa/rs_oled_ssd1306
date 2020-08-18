use crate::*;
use spidev::Spidev;
use std::io::Write;

pub struct SpiClient {
    spi_cli: Spidev,
}

impl Spi for SpiClient {
    fn write(&mut self, data: &[u8]) -> OledSsd1306Result<()> {
        match self.spi_cli.write(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(OledSsd1306ResultError::SpiError(e.to_string())),
        }
    }
}

impl SpiClient {
    pub fn new(spi_cli: Spidev) -> Self {
        Self { spi_cli }
    }
}
