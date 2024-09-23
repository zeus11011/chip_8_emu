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

    pub fn set_pixel(&mut self, x: u8, y: u8, value: u8) -> bool {
        // println!("x{} y{} val {}", x, y, self.pixles[x as usize][y as usize]);
        let normalized_x = x % 32;
        let normalized_y = y % 64;
        let result = self.pixles[normalized_x as usize][normalized_y as usize] ^ value;
        self.pixles[normalized_x as usize][normalized_y as usize] = result;
        return result == 0 || result != value;
    }

    pub fn render(&self) {
        for i in 0..32 {
            for j in 0..64 {
                macroquad::shapes::draw_rectangle(
                    i as f32 * self.scale,
                    j as f32 * self.scale,
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
