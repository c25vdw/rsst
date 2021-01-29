use crate::{Buffer, Result, Subscription};

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
        self.buf.rows = "feed1 feed2 feed3 feed4"
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Ok(())
    }
}
