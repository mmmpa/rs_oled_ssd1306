use eight_px_uint_eight::EightPxUintEightError;

#[cfg(future = "std")]
use poor_gpio::GpioError;

#[derive(Debug, Eq, PartialEq)]
pub enum OledSsd1306ResultError {
    #[cfg(feature = "std")]
    SpiError(String),
    #[cfg(feature = "std")]
    GpioError(String),

    #[cfg(feature = "embedded")]
    SpiError(&'static str),
    #[cfg(feature = "embedded")]
    GpioError(&'static str),

    EightPxUintEightError(EightPxUintEightError),
}

#[cfg(feature = "std")]
impl std::fmt::Display for OledSsd1306ResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OledSsd1306ResultError {}

#[cfg(feature = "std")]
impl From<sysfs_gpio::Error> for OledSsd1306ResultError {
    fn from(e: sysfs_gpio::Error) -> Self {
        Self::GpioError(e.to_string())
    }
}

impl From<EightPxUintEightError> for OledSsd1306ResultError {
    fn from(e: EightPxUintEightError) -> Self {
        Self::EightPxUintEightError(e)
    }
}
