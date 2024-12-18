#[macro_export]
macro_rules! shared_lcd_spi_dma {
    ($peripherals:ident, $dma_channel:expr, $sck:expr, $mosi:expr, $cs:expr) => {
        Spi::new_with_config(
            $peripherals.SPI2,
            esp_hal::spi::master::Config {
                frequency: 40u32.MHz(),
                ..esp_hal::spi::master::Config::default()
            },
        )
        .with_sck($sck)
        .with_mosi($mosi)
        .with_cs($cs)
        .with_dma($dma_channel.configure(false, DmaPriority::Priority0))
    };
}

#[macro_export]
macro_rules! shared_lcd_spi {
    ($peripherals:ident, $sck:expr, $mosi:expr, $cs:expr) => {{
        // Initialize DMA
        let dma = Dma::new($peripherals.DMA);
        let dma_channel = dma.channel0;

        // Return SPI initialized with DMA
        shared_lcd_spi_dma!($peripherals, dma_channel, $sck, $mosi, $cs)
    }};
}

#[macro_export]
macro_rules! shared_lcd_display_interface {
    ($peripherals:ident, $spi:expr, $dc_pin:expr) => {{
        let lcd_dc = Output::new($dc_pin, Level::Low);
        display_interface_spi_dma::new_no_cs(LCD_MEMORY_SIZE, $spi, lcd_dc)
    }};
}

#[macro_export]
macro_rules! shared_lcd_display {
    ($di:expr, $display_model:expr, $width:expr, $height:expr, $orientation:expr, $color_order:expr, $reset_pin:expr) => {{
        mipidsi::Builder::new($display_model, $di)
            .display_size($width, $height)
            .orientation($orientation)
            .color_order($color_order)
            .reset_pin($reset_pin)
    }};
}

pub use {
    shared_lcd_spi_dma, shared_lcd_spi, shared_lcd_display_interface, shared_lcd_display
};
