#[derive(Clone)]
pub struct Buffer {
    pub cx: usize,
    pub cy: usize,
    pub rows: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            cx: 1,
            cy: 1,
            rows: vec![],
        }
    }
}

pub mod consts {
    pub const UP: u8 = 107;
    pub const DOWN: u8 = 106;
    pub const LEFT: u8 = 104;
    pub const RIGHT: u8 = 108;
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
