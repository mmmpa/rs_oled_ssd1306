// https://cdn.sparkfun.com/assets/learn_tutorials/3/0/8/SSD1306.pdf

pub(crate) mod command {
    pub const SET_CONTRAST_CONTROL: u8 = 0x81; // 1-256

    #[repr(u8)]
    pub enum SetEntireDisplayOn {
        ResumeRam = 0xA4,
        IgnoreRam = 0xA5,
    }

    #[repr(u8)]
    pub enum SetDisplayColor {
        Normal = 0xA6,
        Inverse = 0xA7,
    }

    #[repr(u8)]
    pub enum SetDisplayPower {
        Off = 0xAE,
        On = 0xAF,
    }

    pub const SET_MEMORY_ADDRESS_MODE: u8 = 0x20;

    pub const SET_COLUMN_ADDRESS: u8 = 0x21; // data depends on a device
    pub const SET_PAGE_ADDRESS: u8 = 0x22; // data depends on a device

    // Set display RAM display start line register from 0-63 using 0b01xx_xxxx
    // default = 0
    pub const SET_DISPLAY_START_LINE: u8 = 0b0100_0000;

    #[repr(u8)]
    pub enum SetSegmentRemap {
        Column0toSeg0 = 0xA0,
        Column127toSeg0 = 0xA1,
    }

    pub const SET_MULTIPLEX_RATIO: u8 = 0xA8; // data is 0-63 (0x00xxxxxx)

    #[repr(u8)]
    pub enum SetComOutputScanDirection {
        Normal = 0xC0,
        Remapped = 0xC8,
    }

    pub const SET_DISPLAY_OFFSET: u8 = 0xD3; // data is 0-63 (0x00xxxxxx)
    pub const SET_COM_PINS_HARDWARE_CONFIGURATION: u8 = 0xDA;

    // Ratio                = 0b0000_xxxx
    // Oscillator Frequency = 0bxxxx_0000
    //
    // (default 0b1000_0000)
    pub const SET_DISPLAY_CLOCK_DIVIDE_RATIO_OSCILLATOR_FREQUENCY: u8 = 0xD5;

    // Phase 1 = 0b0000xxxx
    // Phase 2 = 0bxxxx0000
    //
    // (default 0b0010_0010)
    pub const SET_PRECHARGE_PERIOD: u8 = 0xD9;

    pub const SET_VCOM_DESELECT_LEVEL: u8 = 0xDB; // enum

    pub const SET_CHARGE_PUMP: u8 = 0x8D;
}

pub(crate) mod data {
    #[repr(u8)]
    pub enum SetMemoryAddressingMode {
        Horizontal = 0b00,
        Vertical = 0b01,
        Page = 0b10,
        Invalid = 0b11,
    }

    pub const SET_COM_PINS_HARDWARE_CONFIGURATION_BASE: u8 = 0b0000_0010;

    #[repr(u8)]
    pub enum SetComPinsHardwareConfigurationSequentialAlternative {
        Sequential = 0b0000_0000,
        Alternative = 0b0001_0000,
    }

    #[repr(u8)]
    pub enum SetComPinsHardwareConfigurationLeftRightRemap {
        Enable = 0b0000_0000,
        Disable = 0b0010_0000,
    }

    #[repr(u8)]
    pub enum SetVcomDeselectLevel {
        UpTo065xVcc = 0x00,
        UpTo077xVcc = 0x20, // default
        UpTo083xVcc = 0x30,
    }

    #[repr(u8)]
    pub enum ChargePumpSetting {
        Enable = 0b0001_0100,
        Disable = 0b0001_0000,
    }
}
