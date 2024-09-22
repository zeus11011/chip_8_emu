use std::u8;

use crate::display;
use rand::Rng;

pub struct cpu {
    display: display::display,
    memory: [u8; 4098],
    registers: [u16; 16],
    index: u16,
    delay_timer: u16,
    sound_timer: u16,
    pc: u16,
    sp: u16,
    stack: Vec<u16>,
    paused: bool,
    speed: u8,
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
            pc: Default::default(),
            sp: Default::default(),
            stack: Default::default(),
            paused: false,
            speed: 10,
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
                }
            }
            0x1000 => {
                self.pc = self.stack.pop().unwrap();
                self.sp -= 1;
            }
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = instruction & 0x0fff;
            }
            0x3000 => {
                let kk = instruction & 0x00ff;
                if self.registers[x] == kk {
                    self.pc += 2
                }
            }
            0x4000 => {
                let kk = instruction & 0x00ff;
                if self.registers[x] != kk {
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
                self.registers[x] = kk;
            }
            0x7000 => {
                let kk = instruction & 0x00ff;
                self.registers[x] += kk;
            }
            0x8000 => match instruction & 0x000f {
                0 => self.registers[x] = self.registers[y],
                1 => self.registers[x] = self.registers[x] | self.registers[y],
                2 => self.registers[x] = self.registers[x] & self.registers[y],
                3 => self.registers[x] = self.registers[x] ^ self.registers[y],
                4 => match self.registers[x].checked_add(self.registers[y]) {
                    Some(sum) => {
                        self.registers[0xf] = 0;
                        self.registers[x] = sum;
                    }
                    None => {
                        self.registers[0xf] = 1;
                        self.registers[x] = 0xffff;
                    }
                },
                5 => match self.registers[x].checked_sub(self.registers[y]) {
                    Some(sum) => {
                        self.registers[x] = sum;
                        self.registers[0xf] = 1
                    }
                    None => {
                        self.registers[0xf] = 0;
                        self.registers[x] = 0x0000
                    }
                },
                6 => {
                    let lsb = self.registers[x] & 0x0001;
                    if lsb == 0x0001 {
                        self.registers[0xf] = 0x0001;
                    } else {
                        self.registers[0xf] = 0x0000;
                    }
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
                    let msb = (self.registers[x] >> 12) & 0x0008;
                    if msb == 0x0008 {
                        self.registers[0xf] = 0x0001;
                    } else {
                        self.registers[0xf] = 0x0000;
                    }
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
                self.pc = self.registers[0] + (instruction & 0x0fff);
            }
            0xC000 => {
                let kk = instruction & 0x00ff;
                let random_num: u8 = rand::thread_rng().gen();
                self.registers[x] = random_num as u16 & kk;
            }
            0xD000 => {
                let mut erased = false;
                let nibble: u16 = instruction & 0x000f;
                for i in 0..nibble {
                    let mut pixel = self.memory[(i + self.index) as usize];
                    for _ in 0..8 {
                        let result = self.display.set_pixel(x as u8, y as u8, pixel >> 7);

                        pixel = pixel << 1;

                        if erased != true && result == true {
                            erased = true;
                            self.registers[0xf] = 1;
                        }
                    }
                }
            }
            0xE000 => {
                todo!();
                let keypress = instruction & 0x00ff;
                if keypress == 0x9e {
                } else if keypress == 0xa1 {
                }
            }
            0xf000 => match instruction & 0x00ff {
                0x07 => {
                    self.registers[x] = self.delay_timer;
                }
                0x0A => {
                    self.paused = true;

                    if true {
                        self.registers[x] = 0;
                        self.paused = false;
                    }
                    todo!();
                }
                0x15 => {
                    self.delay_timer = self.registers[x];
                }
                0x18 => {
                    self.sound_timer = self.registers[x];
                }
                0x1E => {
                    self.index += self.registers[x];
                }
                0x29 => {
                    self.index = (x * 5) as u16;
                }
                0x33 => {
                    self.memory[self.index as usize] = ((x / 100) % 10) as u8;
                    self.memory[self.index as usize + 1] = ((x / 10) % 10) as u8;
                    self.memory[self.index as usize + 2] = (x % 10) as u8;
                }
                0x55 => {
                    for i in 0..x + 1 {
                        self.memory[self.index as usize + i] = self.registers[i] as u8;
                    }
                }
                0x65 => {
                    for i in 0..x + 1 {
                        self.registers[i] = self.memory[self.index as usize + i] as u16;
                    }
                }
                _ => {}
            },
            _ => {
                println!("error")
            }
        }
    }
}
