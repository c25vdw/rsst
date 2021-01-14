use std::io::{self, Read};
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

fn main() {
    let mut one_byte = [0u8; 1];
    let raw_input_mode = RawInputMode::new().unwrap();

    loop {
        match io::stdin().read(&mut one_byte) {
            Ok(n) if n > 0 => {
                // n: the number of bytes read. should be one
                // 0b11111 is CTRL
                // 0x31 is Q. if CTRL+Q exit loop
                if one_byte[0] == 0b11111 & 0x31u8 {
                    break;
                }
                println!("read<{}>: {:?}", n, one_byte);
            }
            _ => (),
        }
    }
}
