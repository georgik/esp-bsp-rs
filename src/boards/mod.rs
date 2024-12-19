#[cfg(feature = "esp32-c3-devkit-rust")]
pub mod esp32_c3_devkit_rust;

#[cfg(feature = "esp32-c3-lcdkit")]
pub mod esp32_c3_lcdkit;

#[cfg(feature = "esp32-c6-devkitc-1")]
pub mod esp32_c6_devkitc_1;

#[cfg(feature = "esp32-s3-box")]
#[macro_use]
pub mod esp32_s3_box;

#[cfg(feature = "esp32-s3-box-3")]
#[macro_use]
pub mod esp32_s3_box_3;

#[cfg(feature = "m5stack-cores3")]
#[macro_use]
pub mod m5stack_cores3;

#[cfg(feature = "waveshare-esp32-c6-lcd-1-47")]
#[macro_use]
pub mod waveshare_esp32_c6_lcd_1_47;

#[cfg(feature = "custom-board")]
#[macro_use]
pub mod custom_board;
