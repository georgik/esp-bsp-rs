#[macro_export]
macro_rules! lcd_spi {
    ($peripherals:ident, $dma_channel:expr) => {
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
}

#[macro_export]
macro_rules! lcd_dma_spi {
    ($peripherals:ident) => {{
        // Initialize DMA
        let dma = Dma::new($peripherals.DMA);
        let dma_channel = dma.channel0;

        // Return SPI initialized with DMA
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
        .with_dma(dma_channel.configure(false, DmaPriority::Priority0))
    }};
}

#[macro_export]
macro_rules! lcd_display_interface {
    ($peripherals:ident, $spi:expr) => {{
        let lcd_dc = Output::new($peripherals.GPIO4, Level::Low);
        display_interface_spi_dma::new_no_cs(LCD_MEMORY_SIZE, $spi, lcd_dc)
    }};
}

#[macro_export]
macro_rules! lcd_reset_pin {
    ($peripherals:ident) => {
        Output::new($peripherals.GPIO48, Level::Low)
    };
}

#[macro_export]
macro_rules! lcd_backlight_init {
    ($peripherals:ident) => {{
        let mut backlight = Output::new($peripherals.GPIO45, Level::Low);
        backlight.set_high();
        Some(backlight)
    }};
}

#[macro_export]
macro_rules! i2c_init {
    ($peripherals:ident) => {{
        let i2c_sda = $peripherals.GPIO8;
        let i2c_scl = $peripherals.GPIO18;
        I2c::new($peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda(i2c_sda)
            .with_scl(i2c_scl)
    }};
}

#[macro_export]
macro_rules! lcd_display {
    ($peripherals:ident, $di:expr) => {{
        mipidsi::Builder::new(mipidsi::models::ILI9342CRgb565, $di)
            .display_size(320, 240)
            // .orientation($orientation)
            .color_order(mipidsi::options::ColorOrder::Bgr)
            .reset_pin(lcd_reset_pin!($peripherals))
    }};
}

pub use {
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_dma_spi, lcd_reset_pin,
    lcd_spi,
};
