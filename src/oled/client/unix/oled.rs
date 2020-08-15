use crate::*;

pub struct OledClient {
    width: u8,
    height: u8,
    spi: SpiClient,
}

impl Oled for OledClient {
    fn height(&self) -> u8 {
        self.height
    }

    fn width(&self) -> u8 {
        self.width
    }
}

impl OledCommand for OledClient {
    type Spi = SpiClient;

    fn spi(&self) -> &Self::Spi {
        &self.spi
    }
}

impl OledClient {
    pub fn new(width: u8, height: u8, spi: SpiClient) -> Self {
        Self { width, height, spi }
    }
}
