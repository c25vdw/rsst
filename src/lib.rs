mod buffer;
mod screen;
mod subscription;

pub use buffer::{Buffer, CursorDir};
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
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let screen = Screen::new().expect("failed to init screen");
        let state = AppState::Subscription;
        Ok(App {
            state,
            screen,
            subscription_controller: SubscriptionsController::new()?,
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
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        match self.state {
            AppState::Subscription => self.screen.render(&self.subscription_controller.buf)?,
            _ => (),
        }
        Ok(())
    }

    fn handle_key_sub(&mut self, byte: u8) {
        match byte {
            106u8 => self
                .subscription_controller
                .buf
                .move_cursor(CursorDir::Down),
            107u8 => self.subscription_controller.buf.move_cursor(CursorDir::Up),
            _ => (),
        }
    }
}
