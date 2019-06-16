use std::fmt;
use std::io::{self, Read, Write};
use std::mem;
use std::str;

/// Clear the screen.
pub const CLEAR: &'static str = "\x1B[2J";

/// Hide the cursor.
pub const HIDE: &'static str = "\x1B[?25l";

/// Show the cursor.
pub const SHOW: &'static str = "\x1B[?25h";

pub const RESET: &'static str = "\x1B[0m";

/// Non-canonical mode terminal.
pub struct Term<'main> {
    termios: libc::termios,
    stdin: io::StdinLock<'main>,
    stdout: io::StdoutLock<'main>,
    buffer: [u8; 1],
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 { return Err(io::Error::last_os_error()) }
    }
}

impl<'main> Term<'main> {
    pub fn new(
        stdin: &'main mut io::Stdin,
        stdout: &'main mut io::Stdout
    ) -> io::Result<Self> {

        let termios = unsafe {

            if libc::isatty(libc::STDIN_FILENO) != 1
            || libc::isatty(libc::STDOUT_FILENO) != 1 {
                return Err(io::Error::new(io::ErrorKind::Other, "[USER ERROR]: not a TTY"))
            }

            // Get current settings
            let mut termios: libc::termios = mem::zeroed();
            test!(libc::tcgetattr(libc::STDIN_FILENO, &mut termios));

            // Change to non-canonical mode
            let mut set = termios.clone();
            set.c_lflag &= !(libc::ICANON | libc::ECHO);
            set.c_cc[libc::VMIN] = 0;
            set.c_cc[libc::VTIME] = 0;
            test!(libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &set));

            // Save for restoring later
            termios
        };

        // Hold onto locks
        let stdin = stdin.lock();
        let mut stdout = stdout.lock();
        write!(stdout, "{}", HIDE)?;
        Ok(Term { termios, stdin, stdout, buffer: [0] })
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        unsafe {
            let mut size: libc::winsize = mem::zeroed();
            test!(libc::ioctl(libc::STDIN_FILENO, libc::TIOCGWINSZ.into(), &mut size));
            Ok((size.ws_col, size.ws_row))
        }
    }

    pub fn poll(&mut self) -> Option<char> {
        if let Ok(_) = self.stdin.read_exact(&mut self.buffer) {
            Some(self.buffer[0] as char)
        } else {
            None
        }
    }
}

impl<'main> io::Write for Term<'main> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<'main> Drop for Term<'main> {
    fn drop(&mut self) {
        unsafe {
            write!(self.stdout, "{}{}{}{}", RESET, CLEAR, Move::default(), SHOW).ok();
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.termios);
        }
    }
}

/// 0-indexed (x, y) terminal position.
#[derive(Copy, Clone, Debug, Default)]
pub struct Move(pub u16, pub u16);

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.1 + 1, self.0 + 1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Paint {
    pub color: Color,
    pub ground: Ground,
}

impl fmt::Display for Paint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let g = self.ground as u8;
        match self.color {
        | Color::C8(c) => write!(fmt, "\x1B[{};5;{}m", g, c.0),
        | Color::C24(c) => write!(fmt, "\x1B[{};2;{};{};{}m", g, c.r, c.g, c.b),
        | Color::Reset => write!(fmt, "\x1B[{}m", g + 1),
        }
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ground {
    Fore = 38,
    Back = 48,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    C8(C8),
    C24(C24),
    Reset,
}

impl str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<u8>() {
            return Ok(Color::C8(C8(c)))
        }

        let mut it = s.split(',');

        let (r, g, b) = match (it.next(), it.next(), it.next(), it.next()) {
        | (Some(r), Some(g), Some(b), None) => (r, g, b),
        | _ => return Err(format!("[USER ERROR]: invalid color specifier {}", s))
        };

        match (r.parse::<u8>(), g.parse::<u8>(), b.parse::<u8>()) {
        | (Ok(r), Ok(g), Ok(b)) => Ok(Color::C24(C24 { r, g, b })),
        | _ => return Err(format!("[USER ERROR]: invalid color specifier {}", s))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C8(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
