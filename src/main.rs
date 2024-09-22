mod display;

// use crate::display;
// mod keyboard;

use std::time::Instant;
mod cpu;
use cpu::cpu;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut d = display::display::default();
    let mut current_time = Instant::now();
    let mut chip_8 = cpu::cpu::default();
    // loop {

    // }
    request_new_screen_size(64.0 * 20.0, 32.0 * 20.0);
    clear_background(BLACK);
    loop {
        if current_time.elapsed().as_millis() > 17 {
            current_time = Instant::now();
            next_frame().await
        }
    }
}
