use crate::{Color, Image, OledSsd1306Result};
use eight_px_uint_eight::{ActAsMono, EightData, EightPxUintEight, Mono, VerticalEightPxUintEight};

pub struct ImageClient<D: EightData + Send + Sync + 'static> {
    pub eight_image: VerticalEightPxUintEight<D>,
}

impl ActAsMono for Color {
    fn act_as(&self) -> Mono {
        match self {
            Color::Light => Mono::One,
            Color::Dark => Mono::Zero,
        }
    }
}

impl<D: EightData + Send + Sync + 'static> ImageClient<D> {
    pub fn with_data(
        width: usize,
        height: usize,
        eight_data: D,
        src: &[Color],
    ) -> OledSsd1306Result<Self> {
        let mut eight_image = VerticalEightPxUintEight::new(width, height, eight_data)?;
        eight_image.update((0, 0, width, height), src)?;

        Ok(Self { eight_image })
    }
}

impl<D: EightData + Send + Sync + 'static> Image for ImageClient<D> {
    fn as_vec(&self) -> &[u8] {
        self.eight_image.as_vec()
    }
}
