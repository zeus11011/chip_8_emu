mod display;

// use crate::display;
// mod keyboard;

use std::{
    fs,
    path::{PathBuf},
    time::Instant,
};
mod cpu;
use macroquad::{prelude::*};
mod keyboard;

use argh::FromArgs;

#[derive(FromArgs)]
/// Chip-8 Emulator implementation in rust
struct Argumentparser {
    ///  file Path of the rom
    #[argh(option, short = 'f')]
    file_path: String,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let arg: Argumentparser = argh::from_env();
    let file_path = PathBuf::from(arg.file_path);
    if !file_path.exists() {
        panic!("File could not be found")
    }

    let rom = fs::read(file_path).unwrap();
    let mut current_time = Instant::now();
    let mut chip_8 = cpu::Cpu::default();

    chip_8.read_rom(rom);
    request_new_screen_size(64.0 * 20.0, 32.0 * 20.0);
    clear_background(BLACK);

    loop {
        if current_time.elapsed().as_millis() > 17 {
            chip_8.cycle().await;
            current_time = Instant::now();
            next_frame().await
        }
    }
}
