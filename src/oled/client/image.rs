use crate::{Color, Image, OledSsd1306Result};
use eight_px_uint_eight::{ActAsMono, Mono, VerticalEightPxUintEight};

pub struct ImageClient {
    eight_image: VerticalEightPxUintEight,
}

impl ActAsMono for Color {
    fn act_as(&self) -> Mono {
        match self {
            Color::Light => Mono::One,
            Color::Dark => Mono::Zero,
        }
    }
}

impl ImageClient {
    pub fn with_data(width: usize, height: usize, src: &[Color]) -> OledSsd1306Result<Self> {
        let eight_image = VerticalEightPxUintEight::with_data(width, height, src)?;

        Ok(Self { eight_image })
    }
}

impl Image for ImageClient {
    fn as_vec(&self) -> &[u8] {
        self.eight_image.as_vec()
    }
}
