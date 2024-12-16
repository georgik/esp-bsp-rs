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

pub use {lcd_spi, lcd_display_interface, lcd_reset_pin, lcd_backlight_init};
