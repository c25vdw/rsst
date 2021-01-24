use std::io::{BufRead, BufReader};
use std::{env::current_dir, vec};

use crate::Buffer;

pub struct SubscriptionsController {
    pub buf: Buffer,
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    url: String,
}

impl From<Vec<Subscription>> for Buffer {
    fn from(items: Vec<Subscription>) -> Self {
        Buffer {
            cx: 1,
            cy: 1,
            rows: items.iter().map(|x| x.url.clone()).collect::<Vec<_>>(),
        }
    }
}

impl SubscriptionsController {
    pub fn debug() -> Self {
        Self {
            buf: Buffer {
                cx: 1,
                cy: 1,
                rows: "hello world what the hell are you doing"
                    .split(' ')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            },
            subscriptions: vec![],
        }
    }

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let curr_dif = current_dir()
            .expect("failed to get current dir")
            .to_str()
            .unwrap()
            .to_string();

        SubscriptionsController::from_url_file(Some(curr_dif + "/urls"))
    }

    pub fn select(&self) -> Subscription {
        self.subscriptions
            .get(self.buf.cy - 0)
            .expect("failed to get the selected subsctription")
            .clone()
    }

    pub fn buf_mut(&mut self) -> &mut Buffer {
        &mut self.buf
    }

    pub fn buf(&self) -> &Buffer {
        &self.buf
    }

    pub fn from_url_file(filepath: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        // https://stackoverflow.com/a/35820003/7358099
        use std::fs;
        if let Some(path) = filepath {
            let file = fs::File::open(path)?;
            let buf = BufReader::new(file);
            let subscriptions = buf
                .lines()
                .filter_map(|l| {
                    if let Ok(url) = l {
                        if valid_url(&url) {
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

fn valid_url(url: &str) -> bool {
    // TODO
    url.len() > 0
}
