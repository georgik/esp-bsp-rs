#[macro_export]
macro_rules! lcd_spi {
    ($peripherals:ident) => {
        shared_lcd_spi!(
            $peripherals,
            $peripherals.GPIO7,  // SCK
            $peripherals.GPIO6,  // MOSI
            $peripherals.GPIO5   // CS
        )
    };
}

#[macro_export]
macro_rules! lcd_display_interface {
    ($peripherals:ident, $spi:expr) => {
        shared_lcd_display_interface!($peripherals, $spi, $peripherals.GPIO4)
    };
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
        I2c::new($peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda($peripherals.GPIO8)
            .with_scl($peripherals.GPIO18)
    }};
}

#[macro_export]
macro_rules! lcd_display {
    ($peripherals:ident, $di:expr) => {
        shared_lcd_display!(
            $di,
            mipidsi::models::ILI9341Rgb565,
            240,
            320,
            mipidsi::options::Orientation::new()
                .flip_vertical()
                .flip_horizontal(),
            mipidsi::options::ColorOrder::Bgr,
            lcd_reset_pin!($peripherals)
        )
    };
}

pub use {
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin,
    lcd_spi,
};
