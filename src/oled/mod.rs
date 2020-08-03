mod client;
mod error;
mod util;
mod value;

pub use client::*;
pub use error::*;
use value::{command, data};

use async_trait::async_trait;
use spidev::Spidev;
use std::io::Write;
use tokio::sync::RwLock;

pub type OledSsd1306Result<T> = Result<T, OledSsd1306ResultError>;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Color {
    Light = 1,
    Dark = 0,
}

pub trait Image: Send + Sync + 'static {
    fn as_vec(&self) -> &[u8];
}

#[async_trait]
pub trait Oled: OledCommand {
    fn height(&self) -> u8;
    fn width(&self) -> u8;
    fn page(&self) -> u8 {
        eight_px_uint_eight::compute_eight_length(self.height() as usize) as u8
    }

    fn sequential_config(&self) -> data::SetComPinsHardwareConfigurationSequentialAlternative {
        if self.height() == 32 || self.height() == 16 {
            data::SetComPinsHardwareConfigurationSequentialAlternative::Sequential
        } else {
            data::SetComPinsHardwareConfigurationSequentialAlternative::Alternative
        }
    }

    async fn init(&self) -> OledSsd1306Result<()> {
        self.reset_flow().await?;

        self.off().await?;

        self.set_memory_address_mode(data::SetMemoryAddressingMode::Horizontal)
            .await?;
        self.set_display_start_line(0x00).await?;
        self.set_segment_remap(command::SetSegmentRemap::Column127toSeg0)
            .await?;
        self.set_multiplex_ratio(self.height() - 1).await?;
        self.set_com_output_scan_direction(command::SetComOutputScanDirection::Remapped)
            .await?;
        self.set_display_offset(0).await?;
        self.set_com_pins_hardware_configuration(
            self.sequential_config(),
            data::SetComPinsHardwareConfigurationLeftRightRemap::Enable,
        )
        .await?;
        self.set_display_clock_divide_ratio_oscillator_frequency(0, 0b1000)
            .await?;
        self.set_precharge_period(0b0001, 0b1111).await?;
        self.set_vcom_deselect_level(data::SetVcomDeselectLevel::UpTo083xVcc)
            .await?;

        self.set_contrast_control(255).await?;
        self.set_entire_display_on(command::SetEntireDisplayOn::ResumeRam)
            .await?;
        self.set_display_color(command::SetDisplayColor::Normal)
            .await?;
        self.set_charge_pump(data::ChargePumpSetting::Enable)
            .await?;

        self.on().await?;

        // width 72 の時に発行される謎のコマンドがある
        if self.width() == 72 {
            self.command(0xAD).await?;
            self.command(0x30).await?;
        }

        self.fill(Color::Dark).await?;

        Ok(())
    }

    async fn fill(&self, color: Color) -> OledSsd1306Result<()> {
        let w = self.width() as usize;
        let p = self.page() as usize;

        let eight_color = match color {
            Color::Light => 0b1111_1111,
            Color::Dark => 0b0000_0000,
        };

        self.draw_vec(&vec![eight_color; (w * p) as usize]).await
    }

    async fn reset_flow(&self) -> OledSsd1306Result<()> {
        self.off().await?;
        self.reset().await?;
        self.on().await?;
        Ok(())
    }

    async fn on(&self) -> OledSsd1306Result<()> {
        self.set_display_power(command::SetDisplayPower::On).await
    }

    async fn off(&self) -> OledSsd1306Result<()> {
        self.set_display_power(command::SetDisplayPower::Off).await
    }

    async fn draw_image<IMG>(&self, image: IMG) -> OledSsd1306Result<()>
    where
        IMG: Image,
    {
        self.draw_vec(image.as_vec()).await
    }

    async fn draw_vec(&self, data: &[u8]) -> OledSsd1306Result<()> {
        self.draw_raw(self.width(), self.page(), data).await
    }
}

// # Data / Command の扱いについて
//
// この LCD ではコマンドでのデータはコマンド直後に送られるコマンドである。
// つまりコマンドに対するデータの送信の際にはデータコマンドピンを使わない。
//
// データモードは画像の転送のみに使われる。
//
// # Command について
//
// 設定を変更できる場合、それぞれの値に一つずつコマンドが割り当てられている場合がある。
// その場合は command 自体を enum として定義している。
#[async_trait]
pub trait OledCommand {
    type Spi: Spi;

    fn spi(&self) -> &Self::Spi;

    async fn command(&self, command: u8) -> OledSsd1306Result<()> {
        self.spi().send_command(command).await
    }

    async fn data(&self, data: &[u8]) -> OledSsd1306Result<()> {
        self.spi().send_data(data).await
    }

    async fn reset(&self) -> OledSsd1306Result<()> {
        self.spi().reset_pin().write(1).await?;
        util::delay_for(10).await;
        self.spi().reset_pin().write(0).await?;
        util::delay_for(100).await;
        self.spi().reset_pin().write(1).await?;
        util::delay_for(100).await;
        Ok(())
    }

    async fn draw_raw(&self, width: u8, page: u8, data: &[u8]) -> OledSsd1306Result<()> {
        let shift = match width {
            64 => 32,
            72 => 24,
            _ => 0,
        };

        self.set_column_address(shift, width - 1 + shift).await?;
        self.set_page_address(0, page - 1).await?;
        self.data(data).await?;
        Ok(())
    }

    async fn set_contrast_control(&self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_CONTRAST_CONTROL).await?;
        self.command(data).await?;
        Ok(())
    }

    async fn set_display_power(&self, com: command::SetDisplayPower) -> OledSsd1306Result<()> {
        self.command(com as u8).await
    }

    async fn set_entire_display_on(
        &self,
        com: command::SetEntireDisplayOn,
    ) -> OledSsd1306Result<()> {
        self.command(com as u8).await
    }

    // light と dark の入れ替え
    async fn set_display_color(&self, com: command::SetDisplayColor) -> OledSsd1306Result<()> {
        self.command(com as u8).await
    }

    async fn set_memory_address_mode(
        &self,
        data: data::SetMemoryAddressingMode,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_MEMORY_ADDRESS_MODE).await?;
        self.command(data as u8).await?;
        Ok(())
    }

    async fn set_column_address(&self, start: u8, end: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_COLUMN_ADDRESS).await?;
        self.command(start).await?;
        self.command(end).await?;
        Ok(())
    }
    async fn set_page_address(&self, start: u8, end: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_PAGE_ADDRESS).await?;
        self.command(start).await?;
        self.command(end).await?;
        Ok(())
    }

    async fn set_display_start_line(&self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_START_LINE).await?;
        self.command(data).await?;
        Ok(())
    }

    async fn set_segment_remap(&self, com: command::SetSegmentRemap) -> OledSsd1306Result<()> {
        self.command(com as u8).await
    }

    async fn set_multiplex_ratio(&self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_MULTIPLEX_RATIO).await?;
        self.command(data).await?;
        Ok(())
    }

    async fn set_com_output_scan_direction(
        &self,
        com: command::SetComOutputScanDirection,
    ) -> OledSsd1306Result<()> {
        self.command(com as u8).await
    }

    async fn set_display_offset(&self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_OFFSET).await?;
        self.command(data).await?;
        Ok(())
    }

    async fn set_com_pins_hardware_configuration(
        &self,
        seq: data::SetComPinsHardwareConfigurationSequentialAlternative,
        remap: data::SetComPinsHardwareConfigurationLeftRightRemap,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_COM_PINS_HARDWARE_CONFIGURATION)
            .await?;
        self.command(data::SET_COM_PINS_HARDWARE_CONFIGURATION_BASE | seq as u8 | remap as u8)
            .await?;
        Ok(())
    }

    async fn set_display_clock_divide_ratio_oscillator_frequency(
        &self,
        ratio: u8,
        freq: u8,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_CLOCK_DIVIDE_RATIO_OSCILLATOR_FREQUENCY)
            .await?;
        self.command(ratio | (freq << 4)).await?;
        Ok(())
    }

    async fn set_precharge_period(&self, phase1: u8, phase2: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_PRECHARGE_PERIOD).await?;
        self.command(phase1 | (phase2 << 4)).await?;
        Ok(())
    }

    async fn set_vcom_deselect_level(
        &self,
        level: data::SetVcomDeselectLevel,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_VCOM_DESELECT_LEVEL).await?;
        self.command(level as u8).await?;
        Ok(())
    }

    async fn set_charge_pump(&self, data: data::ChargePumpSetting) -> OledSsd1306Result<()> {
        self.command(command::SET_CHARGE_PUMP).await?;
        self.command(data as u8).await?;
        Ok(())
    }
}

#[async_trait]
pub trait Spi: Send + Sync + 'static {
    type Pin: Pin;

    fn data_command_pin(&self) -> &Self::Pin;
    fn reset_pin(&self) -> &Self::Pin;
    fn chip_select_pin(&self) -> &Self::Pin;
    fn spi(&self) -> &RwLock<Spidev>;

    async fn use_device(&self) -> OledSsd1306Result<()> {
        self.chip_select_pin().write(0).await?;
        Ok(())
    }

    async fn release_device(&self) -> OledSsd1306Result<()> {
        self.chip_select_pin().write(1).await?;
        Ok(())
    }

    async fn send(&self, data: &[u8]) -> OledSsd1306Result<()> {
        self.spi()
            .write()
            .await
            .write(data)
            .or_else(|e| Err(OledSsd1306ResultError::SpiError(e.to_string())))?;

        Ok(())
    }

    async fn send_command(&self, command: u8) -> OledSsd1306Result<()> {
        debug!("command: 0x{:>02x}", command);
        self.data_command_pin().write(0).await?;
        self.send(&[command]).await?;

        Ok(())
    }

    async fn send_data(&self, data: &[u8]) -> OledSsd1306Result<()> {
        if data.len() == 0 {
            return Ok(());
        }
        debug!("data length: {}", data.len());

        self.data_command_pin().write(1).await?;

        // Buf limit is machine unique. cat /sys/module/spidev/parameters/bufsiz
        for d in data.chunks(4096).into_iter() {
            self.send(d).await?;
        }

        Ok(())
    }
}

#[async_trait]
pub trait Pin: Send + Sync + 'static {
    async fn write(&self, value: u8) -> OledSsd1306Result<()>;
}
