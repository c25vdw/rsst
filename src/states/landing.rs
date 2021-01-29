use crate::{current_dir, BufRead, BufReader, Buffer, CursorDir, Result, State, Subscription};

use super::feeding::Feeding;
use crate::buffer::consts::*;

pub struct Landing {
    pub subscriptions: Vec<Subscription>,
    pub buf: Buffer,
}

impl Landing {
    pub fn new_boxed() -> Result<Box<Self>> {
        let curr_dif = current_dir()
            .expect("failed to get current dir")
            .to_str()
            .unwrap()
            .to_string();

        Ok(Box::new(Self::from_url_file(Some(curr_dif + "/urls"))?))
    }
    pub fn from_url_file(filepath: Option<String>) -> Result<Self> {
        // https://stackoverflow.com/a/35820003/7358099
        use std::fs;
        if let Some(path) = filepath {
            let file = fs::File::open(path)?;
            let buf = BufReader::new(file);
            let subscriptions = buf
                .lines()
                .filter_map(|l| {
                    if let Ok(url) = l {
                        if url.len() > 4 {
                            return Some(Subscription { url });
                        }
                    }
                    None
                })
                .collect::<Vec<Subscription>>();
            return Ok(Self {
                buf: subscriptions.clone().into(),
                subscriptions,
            });
        }
        Ok(Self {
            buf: Buffer {
                cx: 1,
                cy: 1,
                rows: vec![],
            },
            subscriptions: vec![],
        })
    }
}

impl State for Landing {
    fn handle_key_then_next(mut self: Box<Self>, byte: u8) -> Result<Option<Box<dyn State>>> {
        match byte {
            DOWN => self.buf.move_cursor(CursorDir::Down),
            UP => self.buf.move_cursor(CursorDir::Up),
            RIGHT => {
                // self.feed_controller
                //     .load_subscription(self.subscription_controller.select())?
                // self.state = AppState::Feeds;

                let selected_sub = self
                    .subscriptions
                    .get(self.buf.cy - 1)
                    .expect("failed to get the selected subsctription")
                    .clone();
                crate::debug(
                    format!(
                        "selected sub is: {:?}, we are getting the {:?}th sub with cy = {:?}",
                        selected_sub,
                        self.buf.cy - 1,
                        self.buf.cy
                    )
                    .as_str(),
                )?;
                let mut new_state = Feeding {
                    buf: self.buf,
                    subscriptions: self.subscriptions,
                };
                new_state.load_subscription(selected_sub)?;
                return Ok(Some(Box::new(new_state)));
            }
            _ => (),
        }
        Ok(Some(Box::new(*self)))
    }

    fn buf_ref(&self) -> &Buffer {
        &self.buf
    }
}
