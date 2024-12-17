pub use crate::shared::{
    shared_lcd_display, shared_lcd_display_interface,shared_lcd_spi_dma,  shared_lcd_spi,
};

#[cfg(feature = "esp32s3box")]
pub use crate::boards::esp32s3box::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32s3box3")]
pub use crate::boards::esp32s3box3::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "m5stackcores3")]
pub use crate::boards::m5stackcores3::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

pub use crate::boards::custom_board::{
    custom_lcd_spi, custom_lcd_display_interface, custom_lcd_display, custom_i2c_init,
    custom_lcd_backlight, CustomBoardConfig,
};
