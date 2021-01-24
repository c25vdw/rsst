mod buffer;
mod screen;

mod feed;
mod subscription;

pub use buffer::{Buffer, CursorDir};
pub use feed::FeedController;
pub use screen::{RawInputMode, Screen};
pub use subscription::{Subscription, SubscriptionsController};

use std::error::Error;
use std::io::{self, Read};

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
    pub fn new() -> Result<Self, Box<dyn Error>> {
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
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.screen.refresh_screen()?;
        self.screen.get_window_size()?;
        let mut one_byte = [0u8; 1];
        loop {
            match io::stdin().read(&mut one_byte) {
                Ok(n) if n > 0 => {
                    // n: the number of bytes read. should be one
                    // 0b11111 is CTRL
                    // 0x31 is Q. if CTRL+Q exit loop
                    if self.process_key(one_byte[0]).is_err() {
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

    fn process_key(&mut self, byte: u8) -> Result<(), Box<dyn Error>> {
        if byte == 0b11111 & 0x31u8 {
            return Err("exit".into());
        }

        match self.state {
            AppState::Subscription => self.handle_key_sub(byte),
            AppState::Feeds => self.handle_key_feed(byte),
            _ => (),
        };
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        match self.state {
            AppState::Subscription => self.screen.render(&self.subscription_controller.buf)?,
            AppState::Feeds => self.screen.render(&self.feed_controller.buf)?,
            _ => (),
        }
        Ok(())
    }

    fn handle_key_feed(&mut self, byte: u8) {
        match byte {
            106u8 => self.feed_controller.buf.move_cursor(CursorDir::Down),
            107u8 => self.feed_controller.buf.move_cursor(CursorDir::Up),
            108u8 => {}
            _ => (),
        }
    }
    fn handle_key_sub(&mut self, byte: u8) {
        match byte {
            106u8 => self
                .subscription_controller
                .buf
                .move_cursor(CursorDir::Down),
            107u8 => self.subscription_controller.buf.move_cursor(CursorDir::Up),
            108u8 => {
                self.feed_controller
                    .load_subscription(self.subscription_controller.select());
                self.state = AppState::Feeds;
            }
            _ => (),
        }
    }
}
