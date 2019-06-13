use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

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
            Ok((size.ws_col as u16, size.ws_row as u16))
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
        | Color::ANSI(c) => write!(fmt, "\x1B[{};5;{}m", g, c as u8),
        | Color::C256(c) => write!(fmt, "\x1B[{};5;{}m", g, c.0),
        | Color::CRGB(c) => write!(fmt, "\x1B[{};2;{};{};{}m", g, c.r, c.g, c.b),
        | Color::Reset => write!(fmt, "\x1B[{}m", g + 1),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ground {
    Fore = 38,
    Back = 48,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    ANSI(ANSI),
    C256(C256),
    CRGB(CRGB),
    Reset,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ANSI {
    Black         = 00,
    Red           = 01,
    Green         = 02,
    Yellow        = 03,
    Blue          = 04,
    Magenta       = 05,
    Cyan          = 06,
    White         = 07,
    BrightBlack   = 08,
    BrightRed     = 09,
    BrightGreen   = 10,
    BrightYellow  = 11,
    BrightBlue    = 12,
    BrightMagenta = 13,
    BrightCyan    = 14,
    BrightWhite   = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C256(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
