use crate::{Buffer, Channel, Result, Subscription};

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
pub struct FeedController {
    pub buf: Buffer,
}

impl FeedController {
    pub fn new() -> Self {
        FeedController {
            buf: Buffer::default(),
        }
    }

    pub async fn load_subscription(&mut self, sub: Subscription) -> Result<()> {
        let content = reqwest::get(&sub.url).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        // self.write_outputs(format!("{:?}", channel).as_str())?;
        self.buf.rows = channel
            .items()
            .iter()
            .map(|item| item.title().unwrap_or(&"unknown title").to_string())
            .collect::<Vec<_>>();
        Ok(())
    }

    fn write_outputs(&self, out: &str) -> Result<()> {
        let path = Path::new("channel_output.txt");

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(&path)?;

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}
