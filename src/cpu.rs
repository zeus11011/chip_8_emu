use std::u8;

use crate::display;
use crate::keyboard::KeyboardFirm;
use macroquad::input::{get_keys_pressed, get_keys_released};
use rand::Rng;

pub struct Cpu {
    display: display::Display,
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
    pub keyboard: KeyboardFirm,
}

impl Default for Cpu {
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
            keyboard: KeyboardFirm::default(),
        }
    }
}

impl Cpu {
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

                let (sum, _) = self.registers[x].overflowing_add(kk as u8);
                self.registers[x] = sum
            }
            0x8000 => self.exec_8set(instruction, x, y),
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
                                sprite & 0x80,
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

                let pressed_key = self.keyboard.is_key_pressed(self.registers[x] as u8);
                match keypress as u8 {
                    0x9e => {
                        if pressed_key {
                            self.pc += 2;
                        }
                    }
                    0xa1 => {
                        if !pressed_key {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xf000 => {
                self.exec_fset(instruction, x);
            }
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

    fn exec_fset(&mut self, instruction: u16, x: usize) {
        match instruction & 0x00ff {
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
        }
    }

    fn exec_8set(&mut self, instruction: u16, x: usize, y: usize) {
        match instruction & 0x000f {
            0 => self.registers[x] = self.registers[y],
            1 => self.registers[x] = self.registers[x] | self.registers[y],
            2 => self.registers[x] = self.registers[x] & self.registers[y],
            3 => self.registers[x] = self.registers[x] ^ self.registers[y],
            4 => {
                let (sum, bool) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = sum;
                self.registers[0xf] = if bool { 1 } else { 0 };
            }
            5 => {
                self.registers[0xf] = 0;
                if self.registers[x] > self.registers[y] {
                    self.registers[0xf] = 1;
                }
                let (sum, _) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = sum;
            }
            6 => {
                self.registers[0xf] = self.registers[x] & 0x0001;
                self.registers[x] = self.registers[x].checked_div(2).unwrap();
            }
            7 => {
                self.registers[0xf] = 0;
                if self.registers[y] > self.registers[x] {
                    self.registers[0xf] = 1;
                }
                let (sum, _) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = sum;
            }
            0xe => {
                let msb = self.registers[x] >> 7 & 0x0001;
                self.registers[0xf] = msb;
                self.registers[x] = self.registers[x]
                    .checked_mul(2)
                    .unwrap_or_else(|| return 0xff);
            }
            _ => {}
        }
    }

    pub async fn cycle(&mut self) {
        for _ in 0..self.speed {
            self.keyboard.press_key(get_keys_pressed()).await;
            self.keyboard.key_up(get_keys_released()).await;
            let mut instruction: u16 = self.memory[self.pc as usize] as u16;

            instruction = instruction << 8;
            instruction |= self.memory[(self.pc + 1) as usize] as u16;
            if self.paused {
                self.pc = self.pc - 2;
            }

            self.get_op_code(instruction);
            if !self.paused {
                self.update_timer();
            }
        }
        self.display.render();
    }

    fn update_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    #[test]
    fn test_exec8_0() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0;
        cpu.registers[1] = 0x5;
        cpu.exec_8set(0x8000, 0, 1);
        assert_eq!(cpu.registers[0], 0x5);
    }

    #[test]
    fn test_exec8_1() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0;
        cpu.registers[1] = 0xf;
        cpu.exec_8set(0x8001, 0, 1);
        cpu.exec_8set(0x8001, 3, 4);
        assert_eq!(cpu.registers[3], 0x0);
    }
    #[test]
    fn test_exec8_2() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0;
        cpu.registers[1] = 0xf;
        cpu.registers[2] = 0xf;
        cpu.exec_8set(0x8002, 0, 1);
        cpu.exec_8set(0x8002, 2, 1);
        assert_eq!(cpu.registers[0], 0x0);
        assert_eq!(cpu.registers[1], 0xf);
    }
    #[test]
    fn test_exec8_3() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0;
        cpu.registers[1] = 0xf;
        cpu.registers[2] = 0xf;
        cpu.exec_8set(0x8003, 0, 1);
        cpu.exec_8set(0x8003, 2, 1);
        cpu.exec_8set(0x8003, 4, 5);
        assert_eq!(cpu.registers[0], 0xf);
        assert_eq!(cpu.registers[2], 0x0);
        assert_eq!(cpu.registers[4], 0x0);
    }
    #[test]
    fn test_exec8_4() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0;
        cpu.registers[1] = 0xf;
        cpu.exec_8set(0x8004, 0, 1);
        assert_eq!(cpu.registers[0], 0xf);
        assert_eq!(cpu.registers[0xf], 0x0);
    }
    #[test]
    fn test_exec8_4_checkf() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0xff;
        cpu.registers[1] = 0x1;
        cpu.exec_8set(0x8004, 0, 1);
        assert_eq!(cpu.registers[0], 0x0);
        assert_eq!(cpu.registers[0xf], 0x1);
    }
    #[test]
    fn test_exec8_5() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0xff;
        cpu.registers[1] = 0x0f;
        cpu.exec_8set(0x8005, 0, 1);
        assert_eq!(cpu.registers[0], 0xf0);
        assert_eq!(cpu.registers[0xf], 1);
    }
    #[test]
    fn test_exec8_5_vxf() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0x0f;
        cpu.registers[1] = 0xff;
        cpu.exec_8set(0x8005, 0, 1);
        assert_eq!(cpu.registers[0], 16);
        assert_eq!(cpu.registers[0xf], 0);
    }
    #[test]
    fn test_exec8_6() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 7;
        cpu.registers[1] = 0xff;
        cpu.exec_8set(0x8006, 0, 1);
        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[0xf], 1);
    }
    #[test]
    fn test_exec8_6_vxf() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 6;
        cpu.registers[1] = 0xff;
        cpu.exec_8set(0x8006, 0, 1);
        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[0xf], 0);
    }

    #[test]
    fn test_exec8_7() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 7;
        cpu.registers[1] = 0xff;
        cpu.exec_8set(0x8007, 0, 1);
        assert_eq!(cpu.registers[0], 255 - 7);
        assert_eq!(cpu.registers[0xf], 1);
    }

    #[test]
    fn test_exec8_7_vxf() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0xff;
        cpu.registers[1] = 7;
        cpu.exec_8set(0x8007, 0, 1);
        assert_eq!(cpu.registers[0], 8);
        assert_eq!(cpu.registers[0xf], 0);
    }
    #[test]
    fn test_exec8_e() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 4;
        cpu.registers[1] = 7;
        cpu.exec_8set(0x800e, 0, 1);
        assert_eq!(cpu.registers[0], 8);
        assert_eq!(cpu.registers[0xf], 0);
    }
    #[test]
    fn test_exec8_e_vxf() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 0xf0;
        cpu.exec_8set(0x800e, 0, 1);
        assert_eq!(cpu.registers[0], 0xff);
        assert_eq!(cpu.registers[0xf], 1);
    }

    #[test]
    fn test_execf_33() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 123;
        cpu.index = 10;
        cpu.exec_fset(0x0033, 0);
        assert_eq!(cpu.memory[10], 1);
        assert_eq!(cpu.memory[11], 2);
        assert_eq!(cpu.memory[12], 3);
    }
    #[test]
    fn test_execf_33_negative() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 23;
        cpu.index = 10;
        cpu.exec_fset(0x0033, 0);
        assert_eq!(cpu.memory[10], 0);
        assert_eq!(cpu.memory[11], 2);
        assert_eq!(cpu.memory[12], 3);
    }
}
