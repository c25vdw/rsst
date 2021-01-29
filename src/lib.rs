mod buffer;
mod screen;

mod feed;
mod subscription;

pub use anyhow::Result;
pub use buffer::{Buffer, CursorDir};
pub use feed::FeedController;
pub use screen::{RawInputMode, Screen};
pub use subscription::{Subscription, SubscriptionsController};

use anyhow::anyhow;
use rss::Channel;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RsstError {
    #[error("app quit")]
    Quit,
}

enum AppState {
    Subscription,
    Feeds,
    Reading,
}

pub struct App {
    state: AppState,
    screen: Screen, // the screen handler
    subscription_controller: SubscriptionsController,
    feed_controller: FeedController,
}

impl App {
    pub fn new() -> Result<Self> {
        let screen = Screen::new().expect("failed to init screen");
        let state = AppState::Subscription;
        let sub_controller = SubscriptionsController::new()?;
        Ok(App {
            state,
            screen,
            subscription_controller: sub_controller,
            feed_controller: FeedController::new(),
        })
    }
    pub async fn start(&mut self) -> Result<()> {
        self.screen.refresh_screen()?;
        self.screen.get_window_size()?;
        let mut one_byte = [0u8; 1];
        loop {
            match io::stdin().read(&mut one_byte) {
                Ok(n) if n > 0 => {
                    // n: the number of bytes read. should be one
                    // 0b11111 is CTRL
                    // 0x31 is Q. if CTRL+Q exit loop
                    if self.process_key(one_byte[0]).await.is_err() {
                        break;
                    };
                }
                _ => (),
            }
            // println!("key: {:?}", one_byte);
            // return: 13, l: 108
            self.draw()?;
        }
        Ok(())
    }

    async fn process_key(&mut self, byte: u8) -> Result<()> {
        if byte == 0b11111 & 0x31u8 {
            return Err(anyhow!(RsstError::Quit));
        }

        match self.state {
            AppState::Subscription => self.handle_key_sub(byte).await?,
            AppState::Feeds => self.handle_key_feed(byte).await?,
            _ => (),
        };
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        match self.state {
            AppState::Subscription => self.screen.render(&self.subscription_controller.buf)?,
            AppState::Feeds => self.screen.render(&self.feed_controller.buf)?,
            _ => (),
        }
        Ok(())
    }

    async fn handle_key_feed(&mut self, byte: u8) -> Result<()> {
        match byte {
            106u8 => self.feed_controller.buf.move_cursor(CursorDir::Down),
            107u8 => self.feed_controller.buf.move_cursor(CursorDir::Up),
            108u8 => {}
            _ => (),
        }
        Ok(())
    }
    async fn handle_key_sub(&mut self, byte: u8) -> Result<()> {
        match byte {
            106u8 => self
                .subscription_controller
                .buf
                .move_cursor(CursorDir::Down),
            107u8 => self.subscription_controller.buf.move_cursor(CursorDir::Up),
            108u8 => {
                self.feed_controller
                    .load_subscription(self.subscription_controller.select())
                    .await?;
                self.state = AppState::Feeds;
            }
            _ => (),
        }
        Ok(())
    }
}
