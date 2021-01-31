mod buffer;
mod entities;
mod screen;
mod states;

use crate::states::landing::Landing;
pub use anyhow::Result;
pub use buffer::{Buffer, CursorDir};
pub use screen::{RawInputMode, Screen};
pub use states::State;

use rss::Channel;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};

pub fn debug(out: &str) -> Result<()> {
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create("log.txt")?;

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    file.write_all(out.as_bytes())?;
    Ok(())
}

pub fn run() -> Result<()> {
    let mut screen = Screen::new()?;
    let mut state: Box<dyn State> = Landing::new_boxed()?;

    screen.refresh_screen()?;
    screen.get_window_size()?;
    let mut one_byte = [0u8; 1];

    loop {
        match io::stdin().read(&mut one_byte) {
            Ok(n) if n > 0 => {
                if one_byte[0] == 0b11111 & 0x31u8 {
                    break;
                }

                if let Some(next_state) = state.handle_key_then_next(one_byte[0])? {
                    state = next_state;
                } else {
                    break;
                }
            }
            _ => (),
        }

        let buf_ref = state.buf_ref();
        screen.render(buf_ref)?;
    }
    Ok(())
}
