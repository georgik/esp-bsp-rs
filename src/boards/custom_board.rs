/// Generic Custom Board Support
/// This part is in concept stage and does not work
pub struct CustomBoardConfig {
    pub spi_sck: u8,       // GPIO pin for SPI SCK
    pub spi_mosi: u8,      // GPIO pin for SPI MOSI
    pub spi_cs: u8,        // GPIO pin for SPI CS
    pub lcd_dc: u8,        // GPIO pin for LCD DC
    pub lcd_reset: u8,     // GPIO pin for LCD Reset
    pub backlight: Option<u8>, // Optional GPIO for backlight
    pub i2c_sda: u8,       // GPIO pin for I2C SDA
    pub i2c_scl: u8,       // GPIO pin for I2C SCL
}

/// Macro for creating SPI instance for custom boards
#[macro_export]
macro_rules! lcd_spi {
    ($peripherals:ident, $config:expr) => {{
        let dma = Dma::new($peripherals.DMA);
        let dma_channel = dma.channel0;

        Spi::new_with_config(
            $peripherals.SPI2,
            esp_hal::spi::master::Config {
                frequency: 40u32.MHz(),
                ..esp_hal::spi::master::Config::default()
            },
        )
        .with_sck($peripherals.gpio[$config.spi_sck])
        .with_mosi($peripherals.gpio[$config.spi_mosi])
        .with_cs($peripherals.gpio[$config.spi_cs])
        .with_dma(dma_channel.configure(false, DmaPriority::Priority0))
    }};
}

/// Macro for creating Display Interface for custom boards
#[macro_export]
macro_rules! lcd_display_interface {
    ($peripherals:ident, $spi:expr, $config:expr) => {{
        let lcd_dc = Output::new($peripherals.gpio[$config.lcd_dc], Level::Low);
        display_interface_spi_dma::new_no_cs(LCD_MEMORY_SIZE, $spi, lcd_dc)
    }};
}

/// Macro for creating Display with Custom Configuration
#[macro_export]
macro_rules! lcd_display {
    ($peripherals:ident, $di:expr, $config:expr, $model:expr, $width:expr, $height:expr, $orientation:expr, $color_order:expr) => {{
        let lcd_reset = Output::new($peripherals.gpio[$config.lcd_reset], Level::Low);

        mipidsi::Builder::new($model, $di)
            .display_size($width, $height)
            .orientation($orientation)
            .color_order($color_order)
            .reset_pin(lcd_reset)
    }};
}

/// Macro for initializing I2C
#[macro_export]
macro_rules! i2c_init {
    ($peripherals:ident, $config:expr) => {{
        I2c::new($peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda($peripherals.gpio[$config.i2c_sda])
            .with_scl($peripherals.gpio[$config.i2c_scl])
    }};
}

/// Macro for initializing LCD backlight
#[macro_export]
macro_rules! lcd_backlight {
    ($peripherals:ident, $config:expr) => {{
        if let Some(backlight_pin) = $config.backlight {
            let mut backlight = Output::new($peripherals.gpio[backlight_pin], Level::Low);
            backlight.set_high();
            Some(backlight)
        } else {
            None
        }
    }};
}

pub use {
    i2c_init, lcd_display, lcd_display_interface, lcd_backlight,
    lcd_spi,
};