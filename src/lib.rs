#![no_std]

pub enum BoardType {
    ESP32C3LcdKit,
    ESP32C6DevKitC1,
    ESP32S3Box,         // HW was discontinued and replaced by ESP32-S3-BOX-3
    M5StackCoreS3,
    M5StackFire
}

pub struct DisplayConfig {
    pub h_res: u16,
    pub v_res: u16,
}

impl DisplayConfig {
    // Provide a default configuration
    pub fn default() -> DisplayConfig {
        DisplayConfig {
            h_res: 320,
            v_res: 240,
        }
    }

    // Override for specific boards
    pub fn for_board(board: BoardType) -> DisplayConfig {
        match board {
            BoardType::ESP32C3LcdKit => {
                DisplayConfig {
                    h_res: 240,
                    v_res: 240,
                }
            }
            _ => DisplayConfig::default(),
        }
    }
}

#[macro_export]
macro_rules! lcd_gpios {
    (BoardType::ESP32C3LcdKit, $io:ident) => {
        (
            $io.pins.gpio1,     // lcd_sclk
            $io.pins.gpio0,     // lcd_mosi
            $io.pins.gpio7,    // lcd_cs
            $io.pins.gpio4,     // lcd_miso
            $io.pins.gpio2.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio5.into_push_pull_output(),     // lcd_backlight
            $io.pins.gpio8.into_push_pull_output()      // lcd_reset
        )
    };
    (BoardType::ESP32C6DevKitC1, $io:ident) => {
        (
            $io.pins.gpio6,     // lcd_sclk
            $io.pins.gpio7,     // lcd_mosi
            $io.pins.gpio20,    // lcd_cs
            $io.pins.gpio0,     // lcd_miso
            $io.pins.gpio21.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio4.into_push_pull_output(),     // lcd_backlight
            $io.pins.gpio3.into_push_pull_output()      // lcd_reset
        )
    };
    (BoardType::ESP32S3Box, $io:ident) => {
        (
            $io.pins.gpio7,     // lcd_sclk
            $io.pins.gpio6,     // lcd_mosi
            $io.pins.gpio5,    // lcd_cs
            $io.pins.gpio2,     // lcd_miso
            $io.pins.gpio4.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio45.into_push_pull_output(),     // lcd_backlight
            $io.pins.gpio48.into_push_pull_output()      // lcd_reset
        )
    };
    (BoardType::M5StackCoreS3, $io:ident) => {
        (
            $io.pins.gpio36,    // lcd_sclk
            $io.pins.gpio37,    // lcd_mosi
            $io.pins.gpio3,     // lcd_cs
            $io.pins.gpio6,     // lcd_miso
            $io.pins.gpio35.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio0.into_push_pull_output(),    // lcd_backlight
            $io.pins.gpio15.into_push_pull_output()     // lcd_reset
        )
    };
    (BoardType::M5StackFire, $io:ident) => {
        (
            $io.pins.gpio18,    // lcd_sclk
            $io.pins.gpio23,    // lcd_mosi
            $io.pins.gpio5,     // lcd_cs
            $io.pins.gpio19,    // lcd_miso
            $io.pins.gpio26.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio14.into_push_pull_output(),    // lcd_backlight
            $io.pins.gpio27.into_push_pull_output()     // lcd_reset
        )
    };
}

#[macro_export]
macro_rules! define_display_type {
    (BoardType::ESP32C6DevKitC1) => {
        mipidsi::Display<crate::display_interface_spi_dma::SPIInterface<'static, GpioPin<Output<hal::gpio::PushPull>, 21>,
            GpioPin<Output<hal::gpio::PushPull>, 0>,
            hal::peripherals::SPI2, hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9341Rgb565,
            GpioPin<Output<hal::gpio::PushPull>,
            3
        >>
    };
    (BoardType::ESP32S3Box) => {
        mipidsi::Display<crate::display_interface_spi_dma::SPIInterface<'static, GpioPin<Output<hal::gpio::PushPull>, 4>,
            GpioPin<Output<hal::gpio::PushPull>, 0>,
            hal::peripherals::SPI2, hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9342CRgb565,
            GpioPin<Output<hal::gpio::PushPull>,
            48
        >>
    };
    (BoardType::M5StackCoreS3) => {
        mipidsi::Display<crate::display_interface_spi_dma::SPIInterface<'static, GpioPin<Output<esp32s3_hal::gpio::PushPull>, 35>,
            GpioPin<Output<esp32s3_hal::gpio::PushPull>, 0>,
            esp32s3_hal::peripherals::SPI2, esp32s3_hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9342CRgb565,
            GpioPin<Output<esp32s3_hal::gpio::PushPull>,
            15
        >>
    };
}