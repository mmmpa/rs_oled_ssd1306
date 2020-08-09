use eight_px_uint_eight::EightPxUintEightError;
use poor_gpio::GpioError;

#[derive(Debug, Eq, PartialEq)]
pub enum OledSsd1306ResultError {
    SomethingWrong(String),
    SpiError(String),
    GpioError(String),
    EightPxUintEightError(EightPxUintEightError),
}

impl std::fmt::Display for OledSsd1306ResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for OledSsd1306ResultError {}

impl From<GpioError> for OledSsd1306ResultError {
    fn from(e: GpioError) -> Self {
        Self::GpioError(e.to_string())
    }
}

impl From<EightPxUintEightError> for OledSsd1306ResultError {
    fn from(e: EightPxUintEightError) -> Self {
        Self::EightPxUintEightError(e)
    }
}
