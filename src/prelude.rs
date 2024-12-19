pub use crate::shared::{
    shared_lcd_display, shared_lcd_display_interface,shared_lcd_spi_dma,  shared_lcd_spi,
};

#[cfg(feature = "esp32-c3-devkit-rust")]
pub use crate::boards::esp32_c3_devkit_rust::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32-c3-lcdkit")]
pub use crate::boards::esp32_c3_lcdkit::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32-c6-devkitc-1")]
pub use crate::boards::esp32_c6_devkitc_1::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32-s3-box")]
pub use crate::boards::esp32_s3_box::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "esp32-s3-box-3")]
pub use crate::boards::esp32_s3_box_3::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "m5stack-cores3")]
pub use crate::boards::m5stack_cores3::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "waveshare-esp32-c6-lcd-1-47")]
pub use crate::boards::waveshare_esp32_c6_lcd_1_47::{
    i2c_init, lcd_backlight_init, lcd_display, lcd_display_interface, lcd_reset_pin, lcd_spi,
};

#[cfg(feature = "custom-board")]
pub use crate::boards::custom_board::{
    lcd_spi, lcd_display_interface, lcd_display, i2c_init,
    lcd_backlight, CustomBoardConfig,
};
