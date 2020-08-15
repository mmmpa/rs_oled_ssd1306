mod client;
mod error;
mod value;

pub use client::*;
pub use error::*;
pub use value::{command, data};

#[cfg(future = "std")]
mod std_trait;
#[cfg(future = "std")]
pub use std_trait::*;
#[cfg(future = "std")]
mod std_util;

pub type OledSsd1306Result<T> = Result<T, OledSsd1306ResultError>;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Color {
    Light = 1,
    Dark = 0,
}

pub trait Image: Send + Sync {
    fn as_vec(&self) -> &[u8];
}
