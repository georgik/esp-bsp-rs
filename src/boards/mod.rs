#[cfg(feature = "esp32c6devkitc1")]
pub mod esp32c6devkitc1;

#[cfg(feature = "esp32s3box")]
#[macro_use]
pub mod esp32s3box;

#[cfg(feature = "esp32s3box3")]
#[macro_use]
pub mod esp32s3box3;

#[cfg(feature = "m5stackcores3")]
#[macro_use]
pub mod m5stackcores3;


#[cfg(feature = "custom_board")]
#[macro_use]
pub mod custom_board;
