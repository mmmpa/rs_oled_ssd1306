use crate::{OledSsd1306Result, Pin};
use async_trait::async_trait;
use poor_gpio::GpioWriter;

pub struct PinClient {
    cli: poor_gpio::GpioWriterClient,
}

#[async_trait]
impl Pin for PinClient {
    async fn write(&self, value: u8) -> OledSsd1306Result<()> {
        &self.cli.write(value as usize).await?;
        Ok(())
    }
}

impl PinClient {
    pub fn new(cli: poor_gpio::GpioWriterClient) -> Self {
        Self { cli }
    }
}
