use eight_px_uint_eight::EightPxUintEightError;

#[cfg(future = "std")]
use poor_gpio::GpioError;

#[derive(Debug, Eq, PartialEq)]
pub enum OledSsd1306ResultError {
    #[cfg(future = "std")]
    SomethingWrong(String),
    #[cfg(future = "std")]
    SpiError(String),
    #[cfg(future = "std")]
    GpioError(String),
    EightPxUintEightError(EightPxUintEightError),
}

#[cfg(future = "std")]
impl std::fmt::Display for OledSsd1306ResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(future = "std")]
impl std::error::Error for OledSsd1306ResultError {}

#[cfg(future = "std")]
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
