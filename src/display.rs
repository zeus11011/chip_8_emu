use macroquad::color;

pub struct Display {
    pub pixles: Vec<Vec<u8>>,
    scale: f32,
    rows: i32,
    columns: i32,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixles: vec![vec![0; 64]; 32],
            scale: 20.0,
            rows: 32,
            columns: 64,
        }
    }
}

impl Display {
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

#[cfg(test)]
mod test {
    use super::Display;

    #[test]
    fn test_pixel_initialzing() {
        let display = Display::default();
        assert_eq!(display.pixles[0][0], 0)
    }

    #[test]
    fn test_set_pixel() {
        let mut display = Display::default();
        assert_eq!(display.set_pixel(10, 10, 1), false);
        assert_eq!(display.pixles[10][10], 1)
    }

    #[test]
    fn test_set_pixel_out_of_boundary() {
        let mut display = Display::default();
        assert_eq!(display.set_pixel(65, 33, 1), false);
        assert_eq!(display.pixles[1][1], 1);
    }

    #[test]
    fn test_set_pixel_ored() {
        let mut display = Display::default();
        assert_eq!(display.set_pixel(65, 33, 1), false);
        assert_eq!(display.set_pixel(65, 33, 1), true);
    }
}
