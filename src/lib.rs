#![no_std]

pub mod prelude;
pub mod shared;


#[macro_use]
pub mod boards;


pub enum BoardType {
    ESP32C3LcdKit,
    ESP32C6DevKitC1,
    ESP32S3Box,
    M5StackCoreS3,
    M5StackFire,
}

pub struct DisplayConfig {
    pub h_res: u16,
    pub v_res: u16,
}

impl DisplayConfig {
    pub fn default() -> DisplayConfig {
        DisplayConfig {
            h_res: 320,
            v_res: 240,
        }
    }

    pub fn for_board(board: BoardType) -> DisplayConfig {
        match board {
            BoardType::ESP32C3LcdKit => DisplayConfig {
                h_res: 240,
                v_res: 240,
            },
            _ => DisplayConfig::default(),
        }
    }
}
