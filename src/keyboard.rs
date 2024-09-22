use std::collections::HashMap;

use macroquad::input::{
    clear_input_queue, get_char_pressed, get_keys_down, get_keys_pressed, is_key_pressed, KeyCode,
};

pub struct Keyboard_Firm {
    keymap: HashMap<char, u8>,
}

impl Default for Keyboard_Firm {
    fn default() -> Self {
        let mut hash: HashMap<char, u8> = HashMap::new();
        hash.insert('x', 0x0);
        hash.insert('1', 0x1);
        hash.insert('2', 0x2);
        hash.insert('3', 0x3);
        hash.insert('q', 0x4);
        hash.insert('w', 0x5);
        hash.insert('e', 0x6);
        hash.insert('a', 0x7);
        hash.insert('s', 0x8);
        hash.insert('d', 0x9);
        hash.insert('z', 0xA);
        hash.insert('c', 0xB);
        hash.insert('4', 0xc);
        hash.insert('r', 0xD);
        hash.insert('f', 0xE);
        hash.insert('v', 0xf);
        Self { keymap: hash }
    }
}

impl Keyboard_Firm {
    // pub fn isKeyPressed(&mut self, key: i32) -> bool {
    //     let pressed_key = self.k_board_instace.read_key();
    //     // if {

    //     // }
    //     return false;
    // }

    pub fn get_key_pressed(&self) -> u8 {
        // clear_input_queue();
        let char = get_char_pressed();
        match char {
            Some(d) => match self.keymap.get(&d) {
                Some(pressed_key) => return pressed_key.clone(),
                None => return 0,
            },
            None => return 0,
        }
    }
}
