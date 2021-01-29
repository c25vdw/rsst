use crate::{Buffer, Channel, CursorDir, Result, State, Subscription};

use anyhow::anyhow;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::landing::Landing;
use crate::buffer::consts::*;

pub struct FeedController {
    pub buf: Buffer,
}

// impl FeedController {
//     pub fn new() -> Self {
//         FeedController {
//             buf: Buffer::default(),
//         }
//     }
//
//     pub async fn load_subscription(&mut self, sub: Subscription) -> Result<()> {
//         let content = reqwest::get(&sub.url).await?.bytes().await?;
//         let channel = Channel::read_from(&content[..])?;
//
//         // self.write_outputs(format!("{:?}", channel).as_str())?;
//         self.buf.rows = channel
//             .items()
//             .iter()
//             .map(|item| item.title().unwrap_or(&"unknown title").to_string())
//             .collect::<Vec<_>>();
//         Ok(())
//     }
//
//     fn write_outputs(&self, out: &str) -> Result<()> {
//         let path = Path::new("channel_output.txt");
//
//         // Open a file in write-only mode, returns `io::Result<File>`
//         let mut file = File::create(&path)?;
//
//         // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//         file.write_all(out.as_bytes())?;
//         Ok(())
//     }
// }

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
