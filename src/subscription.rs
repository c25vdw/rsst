use std::env::current_dir;
use std::io::{BufRead, BufReader};

use crate::{Buffer, CursorDir};

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

        println!("curr_diff: {:?}", curr_dif);
        SubscriptionsController::from_url_file(Some(curr_dif + "/urls"))
    }
    pub fn from_url_file(filepath: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        // https://stackoverflow.com/a/35820003/7358099
        use std::{env, fs};
        if let Some(path) = filepath {
            let file = fs::File::open(path)?;
            let buf = BufReader::new(file);
            let subscriptions = buf
                .lines()
                .filter_map(|l| {
                    if let Ok(url) = l {
                        Some(Subscription { url })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Subscription>>();
            return Ok(Self {
                buf: subscriptions.clone().into(),
                subscriptions,
            });
        }
        Err("cannot open the file".into())
    }
}
