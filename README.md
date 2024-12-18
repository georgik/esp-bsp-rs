# ESP-BSP-RS

Rust Bare Metal Board Support Packages (BSP) for ESP32-based boards with focus on Embassy Async.

## List of Supported Boards

### Actively Supported Boards

- [ESP32-C6-DevKit-C1](https://docs.espressif.com/projects/espressif-esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/index.html)
- [ESP32-S3-BOX-3](https://github.com/espressif/esp-box)
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
esp-bsp = { version = "0.3.0", features = ["esp32s3box3"] }
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

### Simplified Display Initialization

With `esp_bsp::prelude::*`, the macros ensure correct initialization per board based on the enabled feature.

## Changelog

### 0.3.0

- Unified BSP initialization using shared macros.
- Introduced prelude for simplified imports and initialization.

### 0.2.0

- renamed 
