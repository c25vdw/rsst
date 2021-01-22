pub struct Buffer {
    pub cx: usize,
    pub cy: usize,
    pub rows: Vec<String>,
}

#[derive(Copy, Clone, PartialEq)] // why
pub enum CursorDir {
    Up,
    Down,
}

impl Buffer {
    pub fn move_cursor(&mut self, dir: CursorDir) {
        match dir {
            CursorDir::Up => {
                if self.cy > 1 {
                    self.cy -= 1;
                }
            }
            CursorDir::Down => {
                if self.cy < self.rows.len() {
                    self.cy += 1;
                }
            }
        }
    }
}
