use macroquad::{color, shapes};

pub struct display {
    pixles: [u8; 64 * 32],
    scale: f32,
    rows: i32,
    columns: i32,
}

impl Default for display {
    fn default() -> Self {
        Self {
            pixles: [0; 64 * 32],
            scale: 20.0,
            rows: 32,
            columns: 64,
        }
    }
}

impl display {
    pub fn clear(&mut self) {
        self.pixles = [0; 64 * 32];
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, value: u8) -> bool {
        let normalized_x = x % self.rows as u8;
        let normalized_y = y % self.columns as u8;
        let result = self.pixles
            [(normalized_x as i32 * self.rows + normalized_y as i32 * self.columns) as usize]
            ^ value;
        self.pixles[(x as i32 * self.rows + y as i32 * self.columns) as usize] = result;
        return result != value;
    }

    pub fn render(&self) {
        for i in 0..64 * 32 {
            macroquad::shapes::draw_rectangle(
                (i / self.rows) as f32 * self.scale,
                i as f32 % self.columns as f32 * self.scale,
                self.scale * 2 as f32,
                self.scale,
                if self.pixles[i as usize] == 0 {
                    color::BLACK
                } else {
                    color::WHITE
                },
            );
        }
    }
}
