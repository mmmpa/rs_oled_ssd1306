#![cfg_attr(not(feature = "std"), no_std)]

mod oled;

pub use oled::*;

pub type OledSsd1306Result<T> = Result<T, OledSsd1306ResultError>;
