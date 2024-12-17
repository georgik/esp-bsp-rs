# ESP-BSP-RS

Rust Bare Metal Board Support Packages (BSP) for ESP32-based boards with focus on Embassy Async.

## List of Supported Boards

### Actively Supported Boards

- [ESP32-C6-DevKit-C1](https://docs.espressif.com/projects/espressif-esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/index.html)
- ESP32-S3-BOX-3
- [M5Stack-CoreS3](https://shop.m5stack.com/products/m5stack-cores3-esp32s3-lotdevelopment-kit)

### Older boards

These boards are supported, but not recommended for new projects:

- [ESP32-S3-BOX](https://github.com/espressif/esp-box) - HW discontinued - replaced by ESP32-S3-BOX-3

## Usage

## Adding the BSP to Your Project

To add the ESP-BSP crate to your project:

```
cargo add esp-bsp
```

### Board-Specific Features

Ensure the correct feature flag is enabled in your Cargo.toml:

```toml
[features]
esp32_c6_devkit_c1 = [ "esp32c6-hal", "esp-bsp/esp32c6devkitc1" ]
esp32_s3_box = [ "esp32s3-hal/opsram-8m", "esp-bsp/esp32s3box" ]
esp32_s3_box3 = [ "esp32s3-hal/opsram-8m", "esp-bsp/esp32s3box3" ]
m5stack_cores3 = [ "esp32s3-hal/psram-8m", "esp-bsp/m5stackcores3" ]
```

### Board Initialization

Use the prelude for a streamlined initialization process.

```rust
use esp_bsp::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let mut delay = Delay::new();

    // Initialize I2C for peripherals like accelerometers
    let i2c = i2c_init!(peripherals);

    // Initialize SPI with DMA for LCD display
    let spi = lcd_dma_spi!(peripherals);

    // Create the display interface
    let di = lcd_display_interface!(peripherals, spi);

    // Initialize the display
    let mut display = lcd_display!(peripherals, di)
        .init(&mut delay)
        .unwrap();

    // Turn on the backlight
    lcd_backlight_init!(peripherals);

    // Your application code here
    println!("Display initialized!");
    loop {}
}
```

### Embassy Async Example

If you are using Embassy Async, integrate the display like this:

```rust
#[cfg(feature = "esp32_s3_box")]
type AppDisplay = lcd_display_type!(BoardType::ESP32S3Box);
#[cfg(feature = "m5stack_cores3")]
type AppDisplay = lcd_display_type!(BoardType::M5StackCoreS3);
#[cfg(feature = "esp32_c6_devkit_c1")]
type AppDisplay = lcd_display_type!(BoardType::ESP32C6DevKitC1);

#[embassy_executor::task]
pub async fn app_loop(mut display: AppDisplay) {
    loop {
        // Render something asynchronously
    }
}
```

### Simplified Display Initialization

With `esp_bsp::prelude::*`, the macros ensure correct initialization per board based on the enabled feature.


## Recommended `Cargo.toml` Configuration

Here's a recommended setup for Embassy-based applications with BSP and HAL integration:

```toml
[dependencies]
esp-bsp = "0.2.0"

esp32c6-hal = { version = "0.7.0", optional = true, default-features = false, features = ["embassy", "async", "rt", "embassy-executor-thread"] }
esp32s3-hal = { version = "0.14.0", optional = true, default-features = false, features = ["embassy", "async", "rt", "embassy-executor-thread"] }

[features]
esp32_c6_devkit_c1 = ["esp32c6-hal", "esp-bsp/esp32c6devkitc1"]
esp32_s3_box = ["esp32s3-hal/opsram-8m", "esp-bsp/esp32s3box"]
esp32_s3_box3 = ["esp32s3-hal/opsram-8m", "esp-bsp/esp32s3box3"]
m5stack_cores3 = ["esp32s3-hal/psram-8m", "esp-bsp/m5stackcores3"]
```
## Changelog

### 0.3.0
- Added new esp32_s3_box3 support.
- Unified BSP initialization using shared macros.
- Introduced prelude for simplified imports and initialization.
- Enhanced feature flag support for board-specific configurations.

### 0.2.0

- renamed 
