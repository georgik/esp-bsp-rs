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
macro_rules! with_lcd_spi_pins {
    (BoardType::ESP32S3Box, $peripherals:ident) => {
        (
            .with_sck($peripherals.GPIO7)
            .with_mosi($peripherals.GPIO6)
            .with_cs($peripherals.GPIO5)
        )
    };
    (BoardType::ESP32C6DevKitC1, $peripherals:ident) => {
        (
            .with_sck($peripherals.GPIO6)
            .with_mosi($peripherals.GPIO7)
            .with_cs($peripherals.GPIO20)
        )
    };
    (BoardType::M5StackCoreS3, $peripherals:ident) => {
        (
            .with_sck($peripherals.GPIO36)
            .with_mosi($peripherals.GPIO37)
            .with_cs($peripherals.GPIO3)
        )
    };
    (BoardType::M5StackFire, $peripherals:ident) => {
        (
            .with_sck($peripherals.GPIO18)
            .with_mosi($peripherals.GPIO23)
            .with_cs($peripherals.GPIO5)
        )
    };
}


#[macro_export]
macro_rules! lcd_display_interface {
    ($board:expr, $peripherals:ident, $spi:expr) => {{
        let lcd_dc = match $board {
            BoardType::ESP32S3Box => Output::new($peripherals.GPIO4, Level::Low),
            // BoardType::ESP32C6DevKitC1 => Output::new($peripherals.GPIO21, Level::Low),
            // BoardType::M5StackCoreS3 => Output::new($peripherals.GPIO35, Level::Low),
            _ => panic!("Board not supported!"),
        };

        display_interface_spi_dma::new_no_cs(LCD_MEMORY_SIZE, $spi, lcd_dc)
    }};
}



#[macro_export]
macro_rules! lcd_spi {
    (BoardType::ESP32S3Box, $peripherals:ident, $dma_channel:expr) => {
        Spi::new_with_config(
            $peripherals.SPI2,
            esp_hal::spi::master::Config {
                frequency: 40u32.MHz(),
                ..esp_hal::spi::master::Config::default()
            },
        )
        .with_sck($peripherals.GPIO7)
        .with_mosi($peripherals.GPIO6)
        .with_cs($peripherals.GPIO5)
        .with_dma($dma_channel.configure(false, DmaPriority::Priority0))
    };
    (BoardType::ESP32C6DevKitC1, $peripherals:ident, $dma_channel:expr) => {
        Spi::new_with_config(
            $peripherals.SPI2,
            esp_hal::spi::master::Config {
                frequency: 40u32.MHz(),
                ..esp_hal::spi::master::Config::default()
            },
        )
        .with_sck($peripherals.GPIO6)
        .with_mosi($peripherals.GPIO7)
        .with_cs($peripherals.GPIO20)
        .with_dma($dma_channel.configure(false, DmaPriority::Priority0))
    };
    (BoardType::M5StackCoreS3, $peripherals:ident, $dma_channel:expr) => {
        Spi::new_with_config(
            $peripherals.SPI2,
            esp_hal::spi::master::Config {
                frequency: 40u32.MHz(),
                ..esp_hal::spi::master::Config::default()
            },
        )
        .with_sck($peripherals.GPIO36)
        .with_mosi($peripherals.GPIO37)
        .with_cs($peripherals.GPIO3)
        .with_dma($dma_channel.configure(false, DmaPriority::Priority0))
    };
}

#[macro_export]
macro_rules! lcd_reset_pin {
    ($board:expr, $peripherals:ident) => {
        match $board {
            BoardType::ESP32S3Box => Output::new($peripherals.GPIO48, Level::Low),
            BoardType::ESP32C6DevKitC1 => Output::new($peripherals.GPIO3, Level::Low),
            BoardType::M5StackCoreS3 => Output::new($peripherals.GPIO15, Level::Low),
            _ => panic!("Board not supported!"),
        }
    };
}

#[macro_export]
macro_rules! lcd_backlight_init {
    ($board:expr, $peripherals:ident) => {{
        match $board {
            BoardType::ESP32S3Box => {
                let mut backlight = Output::new($peripherals.GPIO45, Level::Low);
                backlight.set_high(); // Turn on backlight
                Some(backlight)
            }
            // BoardType::ESP32C6DevKitC1 => {
            //     let mut backlight = Output::new($peripherals.GPIO4, Level::Low);
            //     backlight.set_high(); // Turn on backlight
            //     Some(backlight)
            // }
            // BoardType::M5StackCoreS3 => {
            //     let mut backlight = Output::new($peripherals.GPIO0, Level::Low);
            //     backlight.set_high(); // Turn on backlight
            //     Some(backlight)
            // }
            _ => {
                // No backlight control for this board
                None
            }
        }
    }};
}

#[macro_export]
macro_rules! lcd_gpios {
    // (BoardType::ESP32C3LcdKit, $peripherals:ident) => {
    //     (
    //         $peripherals.GPIO1,     // lcd_sclk
    //         $peripherals.GPIO0,     // lcd_mosi
    //         $peripherals.GPIO7,    // lcd_cs
    //         $peripherals.GPIO4,     // lcd_miso
    //         $peripherals.GPIO2.into_push_pull_output(),    // lcd_dc
    //         $peripherals.GPIO5.into_push_pull_output(),     // lcd_backlight
    //         $peripherals.GPIO8.into_push_pull_output()      // lcd_reset
    //     )
    // };
    // (BoardType::ESP32C6DevKitC1, $io:ident) => {
    //     (
    //         $io.pins.gpio6,     // lcd_sclk
    //         $io.pins.gpio7,     // lcd_mosi
    //         $io.pins.gpio20,    // lcd_cs
    //         $io.pins.gpio0,     // lcd_miso
    //         $io.pins.gpio21.into_push_pull_output(),    // lcd_dc
    //         $io.pins.gpio4.into_push_pull_output(),     // lcd_backlight
    //         $io.pins.gpio3.into_push_pull_output()      // lcd_reset
    //     )
    // };
    (BoardType::ESP32S3Box, $peripherals:ident) => {
        (
            $peripherals.GPIO7,     // lcd_sclk
            $peripherals.GPIO6,     // lcd_mosi
            $peripherals.GPIO5,    // lcd_cs
            $peripherals.GPIO2,     // lcd_miso
            Output::new($peripherals.GPIO4, Level::Low),    // lcd_dc
            Output::new($peripherals.GPIO45, Level::Low),     // lcd_backlight
            Output::new($peripherals.GPIO48, Level::Low),      // lcd_reset
        )
    };
    // (BoardType::M5StackCoreS3, $io:ident) => {
    //     (
    //         $io.pins.gpio36,    // lcd_sclk
    //         $io.pins.gpio37,    // lcd_mosi
    //         $io.pins.gpio3,     // lcd_cs
    //         $io.pins.gpio6,     // lcd_miso
    //         $io.pins.gpio35.into_push_pull_output(),    // lcd_dc
    //         $io.pins.gpio0.into_push_pull_output(),    // lcd_backlight
    //         $io.pins.gpio15.into_push_pull_output()     // lcd_reset
    //     )
    // };
    // (BoardType::M5StackFire, $io:ident) => {
    //     (
    //         $io.pins.gpio18,    // lcd_sclk
    //         $io.pins.gpio23,    // lcd_mosi
    //         $io.pins.gpio5,     // lcd_cs
    //         $io.pins.gpio19,    // lcd_miso
    //         $io.pins.gpio26.into_push_pull_output(),    // lcd_dc
    //         $io.pins.gpio14.into_push_pull_output(),    // lcd_backlight
    //         $io.pins.gpio27.into_push_pull_output()     // lcd_reset
    //     )
    // };
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