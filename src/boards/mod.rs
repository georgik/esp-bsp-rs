#[cfg(feature = "esp32c6devkitc1")]
pub use crate::boards::esp32c6devkitc1::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32s3box")]
#[macro_use]
pub mod esp32s3box;

#[cfg(feature = "esp32s3box3")]
#[macro_use]
pub mod esp32s3box3;

#[cfg(feature = "m5stackcores3")]
#[macro_use]
pub mod m5stackcores3;
