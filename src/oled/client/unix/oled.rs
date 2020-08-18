use crate::oled::client::unix::{OutputPinClient, SpiClient};
use crate::*;
use spidev::Spidev;

pub struct OledClient {
    width: u8,
    height: u8,
    data_command_pin: OutputPinClient,
    reset_pin: OutputPinClient,
    chip_select_pin: OutputPinClient,

    spi: SpiClient,
}

impl Oled for OledClient {
    type DataCommandPin = OutputPinClient;
    type ResetPin = OutputPinClient;
    type ChipSelectPin = OutputPinClient;
    type Spi = SpiClient;

    fn data_command_pin(&mut self) -> &mut Self::DataCommandPin {
        &mut self.data_command_pin
    }

    fn reset_pin(&mut self) -> &mut Self::ResetPin {
        &mut self.reset_pin
    }

    fn chip_select_pin(&mut self) -> &mut Self::ChipSelectPin {
        &mut self.chip_select_pin
    }

    fn spi(&mut self) -> &mut Self::Spi {
        &mut self.spi
    }

    fn height(&self) -> u8 {
        self.height
    }

    fn width(&self) -> u8 {
        self.width
    }
}

impl OledClient {
    pub fn new(
        width: u8,
        height: u8,
        data_command_pin: u64,
        reset_pin: u64,
        chip_select_pin: u64,
        spi: Spidev,
    ) -> Self {
        Self {
            width,
            height,
            data_command_pin: OutputPinClient::new(data_command_pin),
            reset_pin: OutputPinClient::new(reset_pin),
            chip_select_pin: OutputPinClient::new(chip_select_pin),
            spi: SpiClient::new(spi),
        }
    }
}
