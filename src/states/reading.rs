use html2text::from_read;

use super::feeding::Feeding;
use crate::buffer::consts::*;
use crate::entities::feed::Feed;
use crate::entities::subscription::Subscription;

use crate::{Buffer, CursorDir, Result, Screen, State};
use textwrap::fill;

pub struct Reading {
    buf: Buffer,
    subscriptions: Vec<Subscription>,
    feedings: Vec<Feed>,
}

impl Reading {
    pub fn from_feeding(old: Box<Feeding>) -> Result<Self> {
        if let Some(selected_feed) = old.feedings.get(old.buf.cy - 1) {
            if let Some(html_content_bytes) = selected_feed
                .item
                .content()
                .and_then(|str_ref| Some(str_ref.as_bytes()))
            {
                let rendered = from_read(html_content_bytes, 10000);
                let (width, _) = {
                    let mut screen = Screen::new()?;
                    screen.get_window_size()?
                };
                let rows = rendered
                    .lines()
                    .flat_map(|l| {
                        // l.to_string()
                        match l.len() {
                            0 => vec![l.to_string()],
                            _ => fill(l, width as usize)
                                .lines()
                                .map(|prev| prev.to_string())
                                .collect::<Vec<_>>(),
                        }
                    })
                    .collect::<Vec<_>>();
                return Ok(Reading {
                    buf: Buffer::from_rows(rows)?,
                    subscriptions: old.subscriptions,
                    feedings: old.feedings,
                });
            }
        }
        Ok(Reading {
            buf: Buffer::default(),
            subscriptions: old.subscriptions,
            feedings: old.feedings,
        })
    }
}

impl State for Reading {
    fn handle_key_then_next(mut self: Box<Self>, byte: u8) -> Result<Option<Box<dyn State>>> {
        match byte {
            DOWN => self.buf.move_cursor(CursorDir::Down),
            UP => self.buf.move_cursor(CursorDir::Up),
            LEFT => {
                let mut prev = Feeding {
                    buf: Buffer::default(),
                    subscriptions: self.subscriptions,
                    feedings: self.feedings,
                };
                prev.bind_buf();
                return Ok(Some(Box::new(prev)));
            }
            _ => (),
        }
        Ok(Some(Box::new(*self)))
    }

    fn bind_buf(&mut self) {}

    fn buf_ref(&self) -> &Buffer {
        &self.buf
    }
}
