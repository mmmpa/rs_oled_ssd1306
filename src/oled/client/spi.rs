use crate::*;
use spidev::Spidev;
use tokio::sync::RwLock;

pub struct SpiClient {
    spidev_cli: RwLock<Spidev>,
    data_command_pin: PinClient,
    reset_pin: PinClient,
    chip_select_pin: PinClient,
}

impl Spi for SpiClient {
    type Pin = PinClient;

    fn data_command_pin(&self) -> &Self::Pin {
        &self.data_command_pin
    }

    fn reset_pin(&self) -> &Self::Pin {
        &self.reset_pin
    }

    fn chip_select_pin(&self) -> &Self::Pin {
        &self.chip_select_pin
    }

    fn spi(&self) -> &RwLock<Spidev> {
        &self.spidev_cli
    }
}

impl SpiClient {
    pub fn new(
        spidev_cli: Spidev,
        data_command_pin: PinClient,
        reset_pin: PinClient,
        chip_select_pin: PinClient,
    ) -> Self {
        Self {
            spidev_cli: RwLock::new(spidev_cli),
            data_command_pin,
            reset_pin,
            chip_select_pin,
        }
    }
}
