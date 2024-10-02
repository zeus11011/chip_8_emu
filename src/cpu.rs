use std::u8;

use crate::display;
use crate::keyboard::Keyboard_Firm;
use macroquad::input::{get_keys_pressed, get_keys_released};
use rand::Rng;

pub struct cpu {
    display: display::display,
    memory: [u8; 4098],
    registers: [u8; 16],
    index: u16,
    delay_timer: u16,
    sound_timer: u16,
    pc: u16,
    sp: u16,
    stack: Vec<u16>,
    paused: bool,
    speed: u8,
    pub keyboard: Keyboard_Firm,
}

impl Default for cpu {
    fn default() -> Self {
        let mut mem: [u8; 4098] = [0; 4098];
        const FONTSET: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, _) in FONTSET.iter().enumerate() {
            mem[i] = FONTSET[i];
        }
        Self {
            display: Default::default(),
            memory: mem,
            registers: Default::default(),
            index: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            pc: 0x200,
            sp: Default::default(),
            stack: Default::default(),
            paused: false,
            speed: 10,
            keyboard: Keyboard_Firm::default(),
        }
    }
}

impl cpu {
    pub fn get_op_code(&mut self, instruction: u16) {
        self.pc += 2;
        let x = ((instruction & 0x0f00) >> 8) as usize;
        let y = ((instruction & 0x00f0) >> 4) as usize;
        match instruction & 0xf000 {
            0x0000 => {
                if instruction == 0x00E0 {
                    self.display.clear();
                } else if instruction == 0x00EE {
                    self.pc = self.stack.pop().unwrap();
                    if self.sp != 0 {
                        self.sp -= 1;
                    }
                }
            }
            0x1000 => {
                self.pc = 0x0fff & instruction;
            }
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = instruction & 0x0fff;
            }
            0x3000 => {
                let kk = instruction & 0x00ff;
                if self.registers[x] == kk as u8 {
                    self.pc += 2
                }
            }
            0x4000 => {
                let kk = instruction & 0x00ff;
                if self.registers[x] != kk as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.registers[x] == self.registers[x] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                let kk = instruction & 0x00ff;
                self.registers[x] = kk as u8;
            }
            0x7000 => {
                let kk = instruction & 0x00ff;
                match self.registers[x].checked_add(kk as u8) {
                    Some(sum) => self.registers[x] = sum,
                    None => self.registers[x] = 0xff,
                };
            }
            0x8000 => match instruction & 0x000f {
                0 => self.registers[x] = self.registers[y],
                1 => self.registers[x] = self.registers[x] | self.registers[y],
                2 => self.registers[x] = self.registers[x] & self.registers[y],
                3 => self.registers[x] = self.registers[x] ^ self.registers[y],
                4 => match (self.registers[x]).checked_add(self.registers[y]) {
                    Some(sum) => {
                        self.registers[0xf] = 0;
                        self.registers[x] = sum;
                    }
                    None => {
                        self.registers[0xf] = 1;
                        self.registers[x] = 0xff;
                    }
                },
                5 => {
                    self.registers[0xf] = 0;
                    if self.registers[x] > self.registers[y] {
                        self.registers[0xf] = 1;
                        self.registers[x] = 0;
                        return;
                    }
                    let (total, _) = self.registers[x].overflowing_sub(self.registers[y]);
                    self.registers[x] = total;
                }
                6 => {
                    self.registers[0xf] = self.registers[x] & 0x0001;
                    self.registers[x] /= 2;
                }
                7 => {
                    if self.registers[y] > self.registers[x] {
                        self.registers[0xf] = 1;
                    } else {
                        self.registers[0xf] = 0;
                    }
                    self.registers[x] = self.registers[y].checked_sub(self.registers[x]).unwrap();
                }
                0xe => {
                    let msb = (self.registers[x] >> 7) & 0x000f;
                    self.registers[0xf] = msb;
                    self.registers[x] *= 2;
                }
                _ => {}
            },
            0x9000 => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.index = instruction & 0x0fff;
            }
            0xB000 => {
                self.pc = self.registers[0] as u16 + (instruction & 0x0fff);
            }
            0xC000 => {
                let kk = instruction & 0x00ff;
                let random_num: u8 = rand::thread_rng().gen();
                self.registers[x] = random_num & kk as u8;
            }
            0xD000 => {
                self.registers[0xf] = 0;
                let witdh: u8 = 8;
                let height: u8 = (instruction & 0x000f) as u8;

                for row in 0..height {
                    let mut sprite = self.memory[(self.index + row as u16) as usize];

                    for col in 0..witdh {
                        if (sprite & 0x80) > 0 {
                            let flipped = self.display.set_pixel(
                                self.registers[x] as u16 + col as u16,
                                self.registers[y] as u16 + row as u16,
                                1,
                            );
                            if flipped {
                                self.registers[0xf] = 1;
                            }
                        }
                        sprite = sprite << 1;
                    }
                }
            }
            0xE000 => {
                let keypress = instruction & 0x00ff;

                if let Some(pressed_key) = self.keyboard.get_key_pressed() {
                    match keypress as u8 {
                        0x9e => {
                            if pressed_key == self.registers[x] {
                                self.pc += 2;
                            }
                        }
                        0xa1 => {
                            if pressed_key != self.registers[x] {
                                self.pc += 2;
                            }
                        }
                        _ => {}
                    }
                    return;
                }
                self.pc -= 2;
            }
            0xf000 => match instruction & 0x00ff {
                0x07 => {
                    self.registers[x] = self.delay_timer as u8;
                }
                0x0A => {
                    self.paused = true;
                    while self.paused {
                        if let Some(get_pressed_key) = self.keyboard.get_key_pressed() {
                            println!("pressed key : {:#02x}", get_pressed_key);
                            self.registers[x] = get_pressed_key;
                            self.paused = false;
                        }
                    }
                }
                0x15 => {
                    self.delay_timer = self.registers[x] as u16;
                }
                0x18 => {
                    self.sound_timer = self.registers[x] as u16;
                }
                0x1E => {
                    self.index += self.registers[x] as u16;
                }
                0x29 => {
                    self.index = (self.registers[x] * 5) as u16;
                }
                0x33 => {
                    self.memory[self.index as usize] = (self.registers[x] / 100) as u8;
                    self.memory[self.index as usize + 1] = ((self.registers[x] % 100) / 10) as u8;
                    self.memory[self.index as usize + 2] = (self.registers[x] % 10) as u8;
                }
                0x55 => {
                    for i in 0..x + 1 {
                        self.memory[self.index as usize + i] = self.registers[i] as u8;
                    }
                }
                0x65 => {
                    for i in 0..x + 1 {
                        self.registers[i] = self.memory[self.index as usize + i];
                    }
                }
                _ => {}
            },
            _ => {
                println!("error")
            }
        }
    }

    pub fn read_rom(&mut self, file_data: Vec<u8>) {
        let initial: u16 = 0x200;
        for (i, data) in file_data.iter().enumerate() {
            self.memory[(initial + i as u16) as usize] = data.clone();
        }
    }

    pub fn cycle(&mut self) {
        // for _ in 0..10 {
        // }
        self.keyboard.press_key(get_keys_pressed());
        self.keyboard.key_up(get_keys_released());
        let mut instruction: u16 = self.memory[self.pc as usize] as u16;

        instruction = instruction << 8;
        instruction |= self.memory[(self.pc + 1) as usize] as u16;
        if self.paused {
            self.pc = self.pc - 2;
        }
        if (self.memory[self.pc as usize] & 0xf0 != 0xe0
            && self.memory[self.pc as usize] & 0x0f == 0x03)
            || (self.memory[(self.pc + 1) as usize] & 0xf0 == 0x40)
        {
            println!("instruction : {:#04x}", instruction);
        }
        self.get_op_code(instruction);
        if !self.paused {
            self.update_timer();
        }
        self.display.render();
    }

    fn update_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
