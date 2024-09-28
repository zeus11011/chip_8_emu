use macroquad::color;

pub struct display {
    pub pixles: Vec<Vec<u8>>,
    scale: f32,
    rows: i32,
    columns: i32,
}

impl Default for display {
    fn default() -> Self {
        Self {
            pixles: vec![vec![0; 64]; 32],
            scale: 20.0,
            rows: 32,
            columns: 64,
        }
    }
}

impl display {
    pub fn clear(&mut self) {
        self.pixles = vec![vec![0; self.columns as usize]; self.rows as usize];
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, value: u8) -> bool {
        let normalized_x = x % 64;
        let normalized_y = y % 32;
        let result = self.pixles[normalized_y as usize][normalized_x as usize] ^ value;
        self.pixles[normalized_y as usize][normalized_x as usize] = result;
        return result == 0 || result != value;
    }

    pub fn render(&self) {
        for i in 0..32 {
            for j in 0..64 {
                macroquad::shapes::draw_rectangle(
                    j as f32 * self.scale,
                    i as f32 * self.scale,
                    self.scale,
                    self.scale,
                    if self.pixles[i][j] == 0 {
                        color::BLACK
                    } else {
                        color::WHITE
                    },
                )
            }
        }
    }
}
