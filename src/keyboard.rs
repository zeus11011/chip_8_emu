use std::collections::{HashMap, HashSet};

use macroquad::input::KeyCode;

pub struct KeyboardFirm {
    pressed_keys: Vec<bool>,
    pub mapped_keys: HashMap<KeyCode, u8>,
}

impl Default for KeyboardFirm {
    fn default() -> Self {
        let mut default_set: HashMap<KeyCode, u8> = HashMap::new();
        default_set.insert(KeyCode::Key1, 0x1);
        default_set.insert(KeyCode::Key2, 0x2);
        default_set.insert(KeyCode::Key3, 0x3);
        default_set.insert(KeyCode::Key4, 0xc);
        default_set.insert(KeyCode::Q, 0x4);
        default_set.insert(KeyCode::W, 0x5);
        default_set.insert(KeyCode::E, 0x6);
        default_set.insert(KeyCode::R, 0xc);
        default_set.insert(KeyCode::A, 0x7);
        default_set.insert(KeyCode::S, 0x8);
        default_set.insert(KeyCode::D, 0x9);
        default_set.insert(KeyCode::F, 0xd);
        default_set.insert(KeyCode::Z, 0xa);
        default_set.insert(KeyCode::X, 0x0);
        default_set.insert(KeyCode::C, 0xb);
        default_set.insert(KeyCode::V, 0xf);

        Self {
            pressed_keys: vec![false; 0xff],
            mapped_keys: default_set,
        }
    }
}

impl KeyboardFirm {
    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        return self.pressed_keys[key as usize];
    }

    pub async fn press_key(&mut self, charset: HashSet<KeyCode>) {
        for (_, char) in charset.iter().enumerate() {
            if let Some(code) = self.mapped_keys.get(&char) {
                self.pressed_keys[*code as usize] = true;
            }
        }
    }

    pub async fn key_up(&mut self, charset: HashSet<KeyCode>) {
        for (_, char) in charset.iter().enumerate() {
            if let Some(code) = self.mapped_keys.get(&char) {
                self.pressed_keys[*code as usize] = false;
            }
        }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        for i in 0..0xf {
            if self.pressed_keys[i as usize] {
                return Some(i as u8);
            }
        }
        return None;
    }
}
