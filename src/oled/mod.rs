mod client;
mod error;
mod value;

pub use client::*;
pub use error::*;
pub use value::{command, data};

use crate::*;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Color {
    Light = 1,
    Dark = 0,
}

pub trait Image: Send + Sync {
    fn as_vec(&self) -> &[u8];
}

pub trait Spi {
    fn write(&mut self, data: &[u8]) -> OledSsd1306Result<()>;
}

pub trait OutputPin {
    fn set_high(&mut self) -> OledSsd1306Result<()>;
    fn set_low(&mut self) -> OledSsd1306Result<()>;
}

pub trait Oled {
    type DataCommandPin: OutputPin;
    type ResetPin: OutputPin;
    type ChipSelectPin: OutputPin;
    type Spi: Spi;

    fn data_command_pin(&mut self) -> &mut Self::DataCommandPin;
    fn reset_pin(&mut self) -> &mut Self::ResetPin;
    fn chip_select_pin(&mut self) -> &mut Self::ChipSelectPin;
    fn spi(&mut self) -> &mut Self::Spi;

    fn height(&self) -> u8;
    fn width(&self) -> u8;

    // Buf limit is machine unique. ex: cat /sys/module/spidev/parameters/bufsiz
    fn buf_limit(&self) -> usize {
        4096
    }

    fn use_device(&mut self) -> OledSsd1306Result<()> {
        self.chip_select_pin().set_low()?;
        Ok(())
    }

    fn release_device(&mut self) -> OledSsd1306Result<()> {
        self.chip_select_pin().set_high()?;
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> OledSsd1306Result<()> {
        self.spi().write(data)?;
        Ok(())
    }

    fn send_command(&mut self, command: u8) -> OledSsd1306Result<()> {
        self.data_command_pin().set_low()?;
        self.send(&[command])?;

        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> OledSsd1306Result<()> {
        if data.len() == 0 {
            return Ok(());
        }

        self.data_command_pin().set_high()?;

        for d in data.chunks(self.buf_limit()).into_iter() {
            self.send(d)?;
        }

        Ok(())
    }

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

    fn init(&mut self) -> OledSsd1306Result<()> {
        self.off()?;

        self.set_memory_address_mode(data::SetMemoryAddressingMode::Horizontal)?;
        self.set_display_start_line(0x00)?;
        self.set_segment_remap(command::SetSegmentRemap::Column127toSeg0)?;
        self.set_multiplex_ratio(self.height() - 1)?;
        self.set_com_output_scan_direction(command::SetComOutputScanDirection::Remapped)?;
        self.set_display_offset(0)?;
        self.set_com_pins_hardware_configuration(
            self.sequential_config(),
            data::SetComPinsHardwareConfigurationLeftRightRemap::Enable,
        )?;
        self.set_display_clock_divide_ratio_oscillator_frequency(0, 0b1000)?;
        self.set_precharge_period(0b0001, 0b1111)?;
        self.set_vcom_deselect_level(data::SetVcomDeselectLevel::UpTo083xVcc)?;

        self.set_contrast_control(255)?;
        self.set_entire_display_on(command::SetEntireDisplayOn::ResumeRam)?;
        self.set_display_color(command::SetDisplayColor::Normal)?;
        self.set_charge_pump(data::ChargePumpSetting::Enable)?;

        self.on()?;

        // width 72 の時に発行される謎のコマンドがある
        if self.width() == 72 {
            self.command(0xAD)?;
            self.command(0x30)?;
        }

        Ok(())
    }

    fn reset(&mut self, mut timer: impl FnMut(usize)) -> OledSsd1306Result<()> {
        self.off()?;
        self.reset_pin().set_high()?;
        timer(100);
        self.reset_pin().set_low()?;
        timer(100);
        self.reset_pin().set_high()?;
        timer(100);
        self.on()?;
        Ok(())
    }

    fn on(&mut self) -> OledSsd1306Result<()> {
        self.set_display_power(command::SetDisplayPower::On)
    }

    fn off(&mut self) -> OledSsd1306Result<()> {
        self.set_display_power(command::SetDisplayPower::Off)
    }

    fn draw_image<IMG>(&mut self, image: &IMG) -> OledSsd1306Result<()>
    where
        IMG: Image,
    {
        self.draw_vec(image.as_vec())
    }

    fn draw_vec(&mut self, data: &[u8]) -> OledSsd1306Result<()> {
        self.draw_flow(self.width(), self.page(), data)
    }

    fn command(&mut self, command: u8) -> OledSsd1306Result<()> {
        self.send_command(command)
    }

    fn data(&mut self, data: &[u8]) -> OledSsd1306Result<()> {
        self.send_data(data)
    }

    fn draw_flow(&mut self, width: u8, page: u8, data: &[u8]) -> OledSsd1306Result<()> {
        let shift = match width {
            64 => 32,
            72 => 24,
            _ => 0,
        };

        self.set_column_address(shift, width - 1 + shift)?;
        self.set_page_address(0, page - 1)?;
        self.data(data)?;
        Ok(())
    }

    fn set_contrast_control(&mut self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_CONTRAST_CONTROL)?;
        self.command(data)?;
        Ok(())
    }

    fn set_display_power(&mut self, com: command::SetDisplayPower) -> OledSsd1306Result<()> {
        self.command(com as u8)
    }

    fn set_entire_display_on(&mut self, com: command::SetEntireDisplayOn) -> OledSsd1306Result<()> {
        self.command(com as u8)
    }

    fn set_display_color(&mut self, com: command::SetDisplayColor) -> OledSsd1306Result<()> {
        self.command(com as u8)
    }

    fn set_memory_address_mode(
        &mut self,
        data: data::SetMemoryAddressingMode,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_MEMORY_ADDRESS_MODE)?;
        self.command(data as u8)?;
        Ok(())
    }

    fn set_column_address(&mut self, start: u8, end: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_COLUMN_ADDRESS)?;
        self.command(start)?;
        self.command(end)?;
        Ok(())
    }
    fn set_page_address(&mut self, start: u8, end: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_PAGE_ADDRESS)?;
        self.command(start)?;
        self.command(end)?;
        Ok(())
    }

    fn set_display_start_line(&mut self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_START_LINE)?;
        self.command(data)?;
        Ok(())
    }

    fn set_segment_remap(&mut self, com: command::SetSegmentRemap) -> OledSsd1306Result<()> {
        self.command(com as u8)
    }

    fn set_multiplex_ratio(&mut self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_MULTIPLEX_RATIO)?;
        self.command(data)?;
        Ok(())
    }

    fn set_com_output_scan_direction(
        &mut self,
        com: command::SetComOutputScanDirection,
    ) -> OledSsd1306Result<()> {
        self.command(com as u8)
    }

    fn set_display_offset(&mut self, data: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_OFFSET)?;
        self.command(data)?;
        Ok(())
    }

    fn set_com_pins_hardware_configuration(
        &mut self,
        seq: data::SetComPinsHardwareConfigurationSequentialAlternative,
        remap: data::SetComPinsHardwareConfigurationLeftRightRemap,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_COM_PINS_HARDWARE_CONFIGURATION)?;
        self.command(data::SET_COM_PINS_HARDWARE_CONFIGURATION_BASE | seq as u8 | remap as u8)?;
        Ok(())
    }

    fn set_display_clock_divide_ratio_oscillator_frequency(
        &mut self,
        ratio: u8,
        freq: u8,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_DISPLAY_CLOCK_DIVIDE_RATIO_OSCILLATOR_FREQUENCY)?;
        self.command(ratio | (freq << 4))?;
        Ok(())
    }

    fn set_precharge_period(&mut self, phase1: u8, phase2: u8) -> OledSsd1306Result<()> {
        self.command(command::SET_PRECHARGE_PERIOD)?;
        self.command(phase1 | (phase2 << 4))?;
        Ok(())
    }

    fn set_vcom_deselect_level(
        &mut self,
        level: data::SetVcomDeselectLevel,
    ) -> OledSsd1306Result<()> {
        self.command(command::SET_VCOM_DESELECT_LEVEL)?;
        self.command(level as u8)?;
        Ok(())
    }

    fn set_charge_pump(&mut self, data: data::ChargePumpSetting) -> OledSsd1306Result<()> {
        self.command(command::SET_CHARGE_PUMP)?;
        self.command(data as u8)?;
        Ok(())
    }
}
