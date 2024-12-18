#[macro_export]
macro_rules! lcd_spi {
    ($peripherals:ident) => {
        shared_lcd_spi!(
            $peripherals,
            $peripherals.GPIO36, // SCK
            $peripherals.GPIO37, // MOSI
            $peripherals.GPIO3   // CS
        )
    };
}

#[macro_export]
macro_rules! lcd_display_interface {
    ($peripherals:ident, $spi:expr) => {
        shared_lcd_display_interface!($peripherals, $spi, $peripherals.GPIO35)
    };
}

#[macro_export]
macro_rules! lcd_reset_pin {
    ($peripherals:ident) => {
        Output::new($peripherals.GPIO15, Level::Low)
    };
}

#[macro_export]
macro_rules! lcd_backlight_init {
    ($peripherals:ident) => {{
        None()
    }};
}

#[macro_export]
macro_rules! i2c_init {
    ($peripherals:ident) => {{
        I2c::new($peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .with_sda($peripherals.GPIO12)
            .with_scl($peripherals.GPIO11)
    }};
}

#[macro_export]
macro_rules! lcd_display {
    ($peripherals:ident, $di:expr) => {
        shared_lcd_display!(
            $di,
            mipidsi::models::ILI9342CRgb565,
            320,
            240,
            mipidsi::options::Orientation::new(),
            mipidsi::options::ColorOrder::Bgr,
            lcd_reset_pin!($peripherals)
        )
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
    };
}

pub use {
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin,
    lcd_spi,
};
