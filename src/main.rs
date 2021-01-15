use std::io::Write;
use std::io::{self, Read};
use std::iter::Iterator;
use std::os::unix::io::AsRawFd; // for .as_raw_fd() implementation

struct RawInputMode {
    stdin: io::Stdin,
    original: termios::Termios,
}

impl RawInputMode {
    fn new() -> Result<Self, io::Error> {
        use termios::*;

        let stdin = io::stdin();
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
        println!("raw input enabled");
        Ok(RawInputMode { stdin, original })
    }
}

impl Drop for RawInputMode {
    fn drop(&mut self) {
        use termios::*;
        tcsetattr(self.stdin.as_raw_fd(), TCSAFLUSH, &self.original)
            .expect("cannot set original TC attributes");
    }
}

struct Screen {
    width: i32,
    height: i32,
    stdout: io::Stdout,
    stdin: io::Stdin,
}

impl Screen {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        use termios::*;
        let stdout = io::stdout();
        let stdin = io::stdin();

        Ok(Screen {
            height: 1,
            width: 1,
            stdout,
            stdin,
        })
    }
    fn get_cursor_position(&mut self) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        self.stdout.write(b"\x1b[6n")?; // this gets the cursor position
        print!("\r\n");

        let mut bytes = [0x20u8; 20]; // that means \x1b col ; row R there is 17 digits for col and row
        match self.stdin.read(&mut bytes) {
            Ok(n) if n > 0 => {
                // example response: <\x1b, \[, 2, 2, \;, 1, 0, 0, 'R', ...spaces>
                let stripped = String::from_utf8(bytes.into())?
                    .replace("\u{1b}", "\u{20}")
                    .replace("[", "\u{20}")
                    .replace("R", "\u{20}")
                    .replace("\u{20}", ""); // "22;100"
                let mut nums_iter = stripped.split(';').collect::<Vec<&str>>().into_iter(); // ["22", "100"]
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

    fn get_window_size(&mut self) -> Result<(i32, i32), io::Error> {
        self.stdout.write(b"\x1b[999C\x1b[999B")?;
        if let Ok((rows, cols)) = self.get_cursor_position() {
            // println!("rows: {}, cols: {}", rows, cols);
            self.height = rows;
            self.width = cols;
        }
        Ok((0, 0))
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        // TODO hide the cursor and use a custom cursor
        self.stdout.write(b"\x1b[2J")?; // this erase the screen
        self.stdout.write(b"\x1b[H")?; // cursor position: default to [1;1H
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut one_byte = [0u8; 1];
    let raw_input_mode = RawInputMode::new().unwrap();
    let mut screen = Screen::new().expect("failed to init screen");

    screen.refresh_screen()?;
    screen.get_window_size()?;
    loop {
        match io::stdin().read(&mut one_byte) {
            Ok(n) if n > 0 => {
                // n: the number of bytes read. should be one
                // 0b11111 is CTRL
                // 0x31 is Q. if CTRL+Q exit loop
                if one_byte[0] == 0b11111 & 0x31u8 {
                    break;
                }
                print!("read<{}>: {:?}, {}\r\n", n, one_byte, one_byte[0] as char);
            }
            _ => (),
        }
    }
    Ok(())
}
