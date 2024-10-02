mod display;

// use crate::display;
// mod keyboard;

use std::{fs, time::Instant};
mod cpu;
use macroquad::prelude::*;
mod keyboard;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut current_time = Instant::now();
    let mut chip_8 = cpu::cpu::default();
    let rom = fs::read("/home/zeus/Documents/roms/chip8-roms/programs/ibm.ch8").unwrap();
    chip_8.read_rom(rom);
    // loop {

    // }
    request_new_screen_size(64.0 * 20.0, 32.0 * 20.0);
    clear_background(BLACK);
    loop {
        if current_time.elapsed().as_millis() > 17 {
            chip_8.cycle();
            current_time = Instant::now();
            next_frame().await
        }
    }
}
