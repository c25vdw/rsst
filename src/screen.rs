use anyhow::Result;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

use crate::Buffer; // for .as_raw_fd() implementation

pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub stdout: io::Stdout,
    pub stdin: io::Stdin,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let stdout = io::stdout();
        let stdin = io::stdin();

        Ok(Screen {
            height: 1,
            width: 1,
            stdout,
            stdin,
        })
    }
    pub fn get_cursor_position(&mut self) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        // Query cursor position
        self.stdout.write(b"\x1b[6n")?;
        print!("\r\n");

        // The buffer to store query results
        let mut bytes = [0x20u8; 10];

        // Parse query results to get rows and cols count
        match self.stdin.read(&mut bytes) {
            Ok(n) if n > 0 => {
                // example response: <\x1b, \[, 2, 2, \;, 1, 0, 0, 'R', ...spaces>
                let stripped = String::from_utf8(bytes.into())?
                    .replace("\u{1b}", "\u{20}")
                    .replace("[", "\u{20}")
                    .replace("R", "\u{20}")
                    .replace("\u{20}", ""); // "22;100"
                let mut nums_iter = stripped.split(';').collect::<Vec<&str>>().into_iter(); // ["22", "100"]

                // Parse str to int
                if let (Some(rows), Some(cols)) = (
                    nums_iter.next().and_then(|x| x.parse::<i32>().ok()),
                    nums_iter.next().and_then(|x| x.parse::<i32>().ok()),
                ) {
                    return Ok((rows, cols));
                }
            }
            _ => (),
        }
        // parse the returning string and get the col, row number
        Err("failed to get dimensions".into())
    }

    pub fn get_window_size(&mut self) -> Result<(i32, i32), io::Error> {
        // Push cursor to the right bottom corner
        self.stdout.write(b"\x1b[9999C\x1b[9999B")?;
        if let Ok((rows, cols)) = self.get_cursor_position() {
            self.height = rows;
            self.width = cols;
        }
        self.stdout.write(b"\x1b[;H")?;
        Ok((0, 0))
    }

    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        // TODO hide the cursor and use a custom cursor
        self.stdout.write(b"\x1b[2J")?;
        self.stdout.write(b"\x1b[H")?; // cursor position: default to [1;1H
        self.stdout.flush()?;
        Ok(())
    }

    fn write_flush(&mut self, bytes: &[u8]) -> Result<(), io::Error> {
        self.stdout.write(bytes)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render(&mut self, buf: &Buffer) -> Result<(), io::Error> {
        // place cursor at the beginning
        self.write_flush(b"\x1b[2J\x1b[1H")?;

        let mut output_buf = Vec::with_capacity((self.height as usize + 2) * self.width as usize);
        // write out all the rows
        for row in buf.rows.iter() {
            // println!("{}\r", row);
            write!(output_buf, "{}\r\n", row)?;
        }
        write!(output_buf, "\x1b[{};{}H", buf.cy, buf.cx)?;
        self.write_flush(&output_buf)?;
        Ok(())
    }
}

pub struct RawInputMode {
    stdin: io::Stdin,
    stdout: io::Stdout,
    original: termios::Termios,
}

impl RawInputMode {
    pub fn new() -> Result<Self, io::Error> {
        use termios::*;

        let stdin = io::stdin();
        let stdout = io::stdout();

        let fd = stdin.as_raw_fd();
        let mut raw = Termios::from_fd(fd)?; // 0 for stdin
        tcgetattr(fd, &mut raw)?;
        let original = raw;

        raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        raw.c_oflag &= !(OPOST);
        raw.c_cflag |= CS8;
        raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        raw.c_cc[VMIN] = 0;
        raw.c_cc[VTIME] = 1;
        tcsetattr(fd, TCSAFLUSH, &raw)?;
        Ok(RawInputMode {
            stdin,
            stdout,
            original,
        })
    }
}

impl Drop for RawInputMode {
    fn drop(&mut self) {
        use termios::*;
        // self.stdout
        //     .write_all(b"\x1b[2J\x1b[1H")
        //     .expect("failed to clean up screen");

        tcsetattr(self.stdin.as_raw_fd(), TCSAFLUSH, &self.original)
            .expect("cannot set original TC attributes");
    }
}
