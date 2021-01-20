pub struct Buffer {
    pub cx: usize,
    pub cy: usize,
    pub rows: Vec<String>,
}

pub struct SubscriptionsController {
    pub buf: Buffer,
}

#[derive(Copy, Clone, PartialEq)] // why
pub enum CursorDir {
    Up,
    Down,
}

impl SubscriptionsController {
    pub fn empty() -> Self {
        Self {
            buf: Buffer {
                cx: 1,
                cy: 1,
                rows: "hello world what the hell are you doing"
                    .split(' ')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            },
        }
    }
    pub fn move_cursor(&mut self, dir: CursorDir) {
        match dir {
            CursorDir::Up => self.buf.cy = self.buf.cy.saturating_sub(1),
            CursorDir::Down => {
                if self.buf.cy < self.buf.rows.len() {
                    self.buf.cy += 1;
                }
            }
        }
    }
}
