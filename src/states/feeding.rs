use crate::{Buffer, Channel, CursorDir, Result, State, Subscription};

use super::landing::Landing;
use crate::buffer::consts::*;
use anyhow::anyhow;

pub struct Feeding {
    pub buf: Buffer,
    pub subscriptions: Vec<Subscription>,
}

impl Feeding {
    pub fn load_subscription(&mut self, sub: Subscription) -> Result<()> {
        if let Ok(content) = reqwest::blocking::get(&sub.url) {
            if let Ok(content) = content.bytes() {
                let channel = Channel::read_from(&content[..])?;

                // self.write_outputs(format!("{:?}", channel).as_str())?;
                self.buf.rows = channel
                    .items()
                    .iter()
                    .map(|item| item.title().unwrap_or(&"unknown title").to_string())
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
            RIGHT => {
                // self.feed_controller
                //     .load_subscription(self.subscription_controller.select())?
                // self.state = AppState::Feeds;
            }
            LEFT => return Ok(Some(Landing::new_boxed()?)),
            _ => (),
        }
        Ok(Some(Box::new(*self)))
    }

    fn buf_ref(&self) -> &Buffer {
        &self.buf
    }
}
