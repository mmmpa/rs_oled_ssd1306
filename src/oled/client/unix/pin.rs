use crate::*;
use sysfs_gpio::{Direction, Pin};

pub struct OutputPinClient {
    cli: Pin,
}

impl OutputPin for OutputPinClient {
    fn set_high(&mut self) -> OledSsd1306Result<()> {
        let led = self.cli;
        led.with_exported(|| {
            led.set_direction(Direction::Out)?;
            led.set_value(1)
        })?;
        Ok(())
    }

    fn set_low(&mut self) -> OledSsd1306Result<()> {
        let led = self.cli;
        led.with_exported(|| {
            led.set_direction(Direction::Out)?;
            led.set_value(0)
        })?;
        Ok(())
    }
}

impl OutputPinClient {
    pub fn new(n: u64) -> Self {
        Self { cli: Pin::new(n) }
    }
}
