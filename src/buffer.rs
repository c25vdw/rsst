use crate::{get_terminal_dimension, Result, Screen};
#[derive(Clone)]
pub struct Buffer {
    pub cx: usize,
    pub cy: usize,
    pub rows: Vec<String>,
    pub display_y_start: usize,
    pub buffer_height: usize,
    _private: (),
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            cx: 1,
            cy: 1,
            rows: vec![],
            display_y_start: 0,
            buffer_height: get_terminal_dimension().ok().unwrap().1,
            _private: (),
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
    pub fn from_rows(rows: Vec<String>) -> Result<Self> {
        let mut screen = Screen::new()?;
        let (_, height) = screen.get_window_size()?;
        Ok(Buffer {
            rows,
            buffer_height: height as usize,
            ..Buffer::default()
        })
    }

    pub fn move_cursor(&mut self, dir: CursorDir) {
        match dir {
            CursorDir::Up => {
                if self.cy > 1 {
                    self.cy -= 1;
                } else {
                    self.display_y_start = self.display_y_start.saturating_sub(1);
                }
            }
            CursorDir::Down => {
                if self.cy < self.rows.len() && self.cy < self.buffer_height {
                    self.cy += 1;
                } else if self.display_y_start + self.buffer_height + 1 < self.rows.len() {
                    self.display_y_start += 1;
                }
            }
        }
    }
}
