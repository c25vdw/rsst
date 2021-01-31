use crate::{Buffer, Channel, CursorDir, Result, State};

use super::landing::Landing;
use super::reading::Reading;
use crate::buffer::consts::*;
use crate::entities::feed::Feed;
use crate::entities::subscription::Subscription;
use anyhow::anyhow;

pub struct Feeding {
    pub buf: Buffer,
    pub subscriptions: Vec<Subscription>,
    pub feedings: Vec<Feed>,
}

impl Feeding {
    pub fn load_subscription(&mut self, sub: Subscription) -> Result<()> {
        if let Ok(content) = reqwest::blocking::get(&sub.url) {
            if let Ok(content) = content.bytes() {
                let channel = Channel::read_from(&content[..])?;
                self.feedings = channel
                    .items()
                    .iter()
                    .map(|item| Feed { item: item.clone() })
                    .collect::<Vec<_>>();

                Ok(())
            } else {
                Err(anyhow!("failed to load bytes".to_string()))
            }
        } else {
            Err(anyhow!("failed to load sub".to_string()))
        }
    }
}

impl State for Feeding {
    fn handle_key_then_next(mut self: Box<Self>, byte: u8) -> Result<Option<Box<dyn State>>> {
        match byte {
            DOWN => self.buf.move_cursor(CursorDir::Down),
            UP => self.buf.move_cursor(CursorDir::Up),
            RIGHT => return Ok(Some(Box::new(Reading::from_feeding(self)))),
            LEFT => {
                let mut prev = Landing {
                    buf: Buffer::default(),
                    subscriptions: self.subscriptions,
                };
                prev.bind_buf();
                return Ok(Some(Box::new(prev)));
            }
            _ => (),
        }
        Ok(Some(Box::new(*self)))
    }

    fn bind_buf(&mut self) {
        self.buf.rows = self
            .feedings
            .iter()
            .map(|feed| feed.item.title().unwrap_or(&"unknown title").to_string())
            .collect::<Vec<_>>();
    }

    fn buf_ref(&self) -> &Buffer {
        &self.buf
    }
}
